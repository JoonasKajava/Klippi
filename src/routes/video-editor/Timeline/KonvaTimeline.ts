import Konva from 'konva';
import type { KonvaEventObject } from 'konva/lib/Node';
import { join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { clipEnd, clipStart } from '$lib/stores/ClipOptionsStore';

const clamp = (num: number, min: number, max: number) => Math.min(Math.max(num, min), max);
const SCROLL_LEEWAY = 50;

const DETAIL_CUTOFF_SCALE = 80;

const zoomSpeed = 1.1;

Konva.dragButtons = [2];
const DETAIL_NAME = 'detail';
const CLIP_MARKER_NAME = 'clip_marker';
const TIME_MARKER_NAME = 'time_marker';
const THUMBNAIL_NAME = 'thumbnail';
const CLIP_MARKER_ANCHOR_NAME = '_anchor';

export class KonvaTimeline {
    private _width: number;
    private _height: number;

    private readonly _marker_gap: number;
    private readonly _seconds_in_timeline: number;
    private _current_time: number = 0;

    private readonly _stage: Konva.Stage;

    private readonly _timeline_group: Konva.Group

    private readonly _main_layer: Konva.Layer

    private readonly _current_time_marker: Konva.Rect;

    private _clip_marker: Konva.Rect | undefined;
    private _clip_marker_transformer: Konva.Transformer | undefined

    private _was_controlling_clip_marker = false;

    private _mouse_drag_start: number | null = null;

    private _created_thumbnails: number = 1;

    private readonly _on_select_current_time: (time: number) => void;

    constructor(width: number, height: number, secondsInTimeline: number, currentTimeUpdate: (time: number) => void) {
        this._width = width;
        this._height = height;
        this._seconds_in_timeline = secondsInTimeline;
        this._marker_gap = width / secondsInTimeline;
        this._on_select_current_time = currentTimeUpdate;
        this._main_layer = this.create_main_layer();
        this._stage = this.create_stage(this._main_layer);
        this._timeline_group = this.create_timeline_group(this._main_layer);
        this._current_time_marker = this.create_current_time_marker(this._timeline_group);

        for (let i = 0; i < secondsInTimeline; i++) {
            this.create_time_marker(i, this._timeline_group);
        }
    }

    get_scale() {
        if (!this._stage) return 1;
        return this._stage.scaleX();
    }

    get_marker_gap() {
        return this._marker_gap;
    }

    update_clip_marker(start: number, duration: number) {
        if (!this._clip_marker) this.create_clip_marker(this._timeline_group);

        this._clip_marker?.x(start * this._marker_gap);
        this._clip_marker?.width(duration * this._marker_gap);
    }

    timestamp_to_string(timestamp: number): string {
        return `${Math.floor(timestamp / 60)}:${(timestamp % 60)
            .toFixed(0)
            .padStart(2, '0')}`;
    }

    select_current_time(e: KonvaEventObject<WheelEvent>) {
        if (this._was_controlling_clip_marker) {
            this._clip_marker?.stopDrag();
            this._was_controlling_clip_marker = false;
            return;
        }
        if (e.evt.button !== 0) return;
        if (!this._stage) {
            console.log('stage not created');
            return;
        }
        this._on_select_current_time(this._stage.getRelativePointerPosition().x / this._marker_gap);
    }

    start_clip_marker_change(e: KonvaEventObject<MouseEvent>) {
        if (e.evt.button !== 0) return;
        if (!this._clip_marker) {
            this._clip_marker = this.create_clip_marker(this._timeline_group);
        }

        if (e.target.name() === CLIP_MARKER_NAME) {
            this._clip_marker.startDrag();
            this._was_controlling_clip_marker = true;
            return;
        }

        if (e.target.name().includes(CLIP_MARKER_ANCHOR_NAME)) {
            this._was_controlling_clip_marker = true;
            return;
        }
        this._mouse_drag_start = this._stage.getRelativePointerPosition().x / this._marker_gap;
    }

    move_clip_marker() {
        if (!this._mouse_drag_start) return;
        if (!this._stage) {
            console.log('stage not created');
            return;
        }
        if (!this._clip_marker) {
            console.log('clip marker not created');
            return;
        }

        const rangeEnd = this._stage.getRelativePointerPosition().x / this._marker_gap;
        const distance = Math.abs(this._mouse_drag_start - rangeEnd);
        if (distance > 0.1) {
            this._clip_marker.x(
                (rangeEnd < this._mouse_drag_start ? rangeEnd : this._mouse_drag_start) *
                this._marker_gap
            );
            this._clip_marker.width(distance * this._marker_gap);
            this._was_controlling_clip_marker = true;
        }
    }

    complete_clip_marker_change() {
        if (!this._clip_marker) return;

        const duration = this._clip_marker.getSelfRect().width / this._marker_gap;
        const start = this._clip_marker.x() / this._marker_gap;

        this._mouse_drag_start = null;
        if (this._clip_marker.width() > 0) {
            clipStart.set(start);
            clipEnd.set(start + duration);
        }
    }

    stage_horizontal_bound(x: number) {
        if (!this._stage) return 0;
        const rightBound =
            -(this._seconds_in_timeline * this._marker_gap * this._stage.scaleX()) +
            this._stage.width() -
            SCROLL_LEEWAY;
        return clamp(x, rightBound, SCROLL_LEEWAY);
    }

    stage_set_scale(scale: number, stagePosition: number) {
        if (!this._stage) {
            console.log('stage not created');
            return;
        }
        if (!this._timeline_group) {
            console.log('timeline group not created');
            return;
        }
        this._stage.scale({ x: scale, y: 1 });

        this._stage.position({
            x: stagePosition,
            y: 0
        });

        const children = this._timeline_group.getChildren();
        for (const child of children) {
            if (child.name() === CLIP_MARKER_NAME) continue;
            if (child.name().includes(THUMBNAIL_NAME)) continue;
            child.scaleX(1 / scale);
        }

        this.update_detail_visibility()
    }

    stage_zoom(e: KonvaEventObject<WheelEvent>) {
        e.evt.preventDefault();
        if (!this._stage) {
            console.log('stage not created');
            return;
        }

        const oldScale = this._stage.scaleX();

        const pointer = this._stage.getPointerPosition();
        if (!pointer) {
            console.log('pointer not found');
            return;
        }
        const mousePointTo = {
            x: (pointer.x - this._stage.x()) / oldScale,
            y: (pointer.y - this._stage.y()) / oldScale
        };

        const direction = e.evt.deltaY < 0 ? 1 : -1;

        const newScale =
            direction > 0 ? oldScale * zoomSpeed : oldScale / zoomSpeed;

        if (newScale <= 1) return;

        this.stage_set_scale(newScale, pointer.x - mousePointTo.x * newScale);
    }

    update_stage_dimensions(width: number, height: number) {
        this._width = width;
        this._height = height;

        if (!this._stage) return;
        this._stage.width(width);
        this._stage.height(height);
    }

    update_current_time(currentTime: number) {
        if (!this._current_time_marker) return;
        this._current_time_marker.x(currentTime * this._marker_gap);
        this._current_time = currentTime;
    }

    update_detail_visibility() {
        if (!this._timeline_group) {
            console.log('timeline group not created');
            return;
        }
        if (!this._stage) {
            console.log('stage not created');
            return;
        }
        const details = this._timeline_group.getChildren((e) => e.name().includes(DETAIL_NAME))
        for (const detail of details) {
            detail.visible(this._stage.scaleX() > DETAIL_CUTOFF_SCALE);
        }
    }

    create_timeline_group(layer: Konva.Layer): Konva.Group {
        const timelineGroup = new Konva.Group();
        layer.add(timelineGroup);
        return timelineGroup;
    }

    create_clip_marker_transformer(clipMarker: Konva.Rect, timelineGroup: Konva.Group): Konva.Transformer {
        this._clip_marker_transformer = new Konva.Transformer({
            name: CLIP_MARKER_NAME,
            nodes: [clipMarker],
            centeredScaling: false,
            rotateEnabled: false,
            enabledAnchors: ['middle-right'],
            boundBoxFunc: (oldBox, newBox) => {
                const scaleX = this._stage?.scaleX();
                if (!scaleX) return oldBox;
                this._clip_marker?.width(newBox.width / scaleX);

                return oldBox;
            }
        });

        timelineGroup.add(this._clip_marker_transformer);
        return this._clip_marker_transformer;
    }

    create_time_marker(timestamp: number, timelineGroup: Konva.Group): Konva.Rect | null {
        const mark = new Konva.Rect({
            name: TIME_MARKER_NAME,
            fill: 'black',
            x: timestamp * this._marker_gap,
            y: 0,
            height: this._height,
            width: 1
        });

        const text = new Konva.Text({
            name: DETAIL_NAME,
            fill: 'black',
            x: timestamp * this._marker_gap,
            y: this._height,
            text: this.timestamp_to_string(timestamp),
            fontSize: 15,
            fontFamily: 'Calibri',
            visible: false
        });

        text.offsetX(text.width() / 2);
        text.offsetY(text.height());

        mark.height(this._height - text.height())
        mark.listening(false);
        text.listening(false);
        mark.perfectDrawEnabled(false);
        text.perfectDrawEnabled(false);

        timelineGroup.add(mark);
        timelineGroup.add(text);

        return mark;
    }

    create_clip_marker(timelineGroup: Konva.Group): Konva.Rect {
        const clipMarker = new Konva.Rect({
            name: CLIP_MARKER_NAME,
            fill: 'blue',
            x: 0,
            y: this._height * 0.1,
            height: this._height * 0.5,
            width: 0,
            dragBoundFunc: function (pos) {
                return {
                    x: pos.x,
                    y: this.absolutePosition().y
                };
            }
        });
        timelineGroup.add(clipMarker);
        this.create_clip_marker_transformer(clipMarker, timelineGroup);
        return clipMarker
    }

    create_current_time_marker(timelineGroup: Konva.Group) {
        const currentTimeLineMarker = new Konva.Rect({
            fill: 'red',
            x: this._current_time * this._marker_gap,
            y: 0,
            height: this._height,
            width: 1
        });

        timelineGroup.add(currentTimeLineMarker);

        return currentTimeLineMarker;
    }

    create_main_layer(): Konva.Layer {
        return new Konva.Layer();
    }

    create_stage(layer: Konva.Layer): Konva.Stage {
        const stage = new Konva.Stage({
            container: 'stage',
            width: this._width,
            height: this._height,
            draggable: true,
            dragBoundFunc: (pos) => {
                return {
                    x: this.stage_horizontal_bound(pos.x),
                    y: this._stage?.absolutePosition().y ?? 0
                };
            }
        });

        stage.add(layer);

        stage.on('wheel', this.stage_zoom.bind(this));
        stage.on('pointerclick', this.select_current_time.bind(this));
        stage.on('pointerdown', this.start_clip_marker_change.bind(this));
        stage.on('pointermove', this.move_clip_marker.bind(this));
        stage.on('pointerup', this.complete_clip_marker_change.bind(this));

        return stage;
    }

    async createTimelineThumbnail(thumbnailPath: string, index: number) {
        const fullPath = await join(thumbnailPath, `${index}.bmp`);

        console.log(index);
        if (!this._current_time_marker) {
            console.log('current time marker not created');
            return;
        }
        Konva.Image.fromURL(convertFileSrc(fullPath), (image) => {
            if (!this._timeline_group) {
                console.log('timeline group not created');
                return;
            }
            image.setAttrs({
                name: `${THUMBNAIL_NAME} ${DETAIL_NAME}`,
                x: (index - 2) * this._marker_gap,
                y: 0,
                height: this._height * 0.7,
                width: this._marker_gap,
                visible: true
            });
            image.listening(false);
            image.perfectDrawEnabled(false);
            this._timeline_group.add(image);
        })
        this._current_time_marker.zIndex(0);
    }

    create_timeline_thumbnails(thumbnailPath: string, upto: number) {
        for (let i = this._created_thumbnails; i < upto; i++) {
            this.createTimelineThumbnail(thumbnailPath, i).catch((e) => {
                console.log(e)
            });
        }
        this._created_thumbnails = upto;
    }
}
