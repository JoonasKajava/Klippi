import Konva from "konva";
import type { KonvaEventObject } from "konva/lib/Node";
import { clip_end, clip_start } from "../ClipOptions/ClipOptionsStore";
import { join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";


const clamp = (num, min, max) => Math.min(Math.max(num, min), max);
const SCROLL_LEEWAY = 50;

const DETAIL_CUTOFF_SCALE = 80;

const zoom_speed = 1.1;

Konva.dragButtons = [2];
const DETAIL_NAME = "detail";
const CLIP_MARKER_NAME = "clip_marker";
const TIME_MARKER_NAME = "time_marker";
const THUMBNAIL_NAME = "thumbnail";
const CLIP_MARKER_ANCHOR_NAME = "_anchor";


export class KonvaTimeline {

    private _width: number;
    private _height: number;

    private _marker_gap: number;
    private _seconds_in_timeline: number;
    private _current_time: number;

    private _stage: Konva.Stage;

    private _timeline_group: Konva.Group;

    private _main_layer: Konva.Layer;

    private _current_time_marker: Konva.Rect;

    private _clip_marker: Konva.Rect;
    private _clip_marker_transformer: Konva.Transformer;

    private _was_controlling_clip_marker = false;

    private _mouse_drag_start: number;

    private _created_thumbnails: number = 1;

    private _on_select_current_time: (time: number) => void;

    constructor(width: number, height: number, seconds_in_timeline: number, current_time_update: (time: number) => void) {
        this._width = width;
        this._height = height;
        this._seconds_in_timeline = seconds_in_timeline;
        this._marker_gap = width / seconds_in_timeline;
        this._on_select_current_time = current_time_update;
    }



    get_scale() {
        return this._stage.scaleX();
    }

    get_marker_gap() {
        return this._marker_gap;
    }

    update_clip_marker(start: number, duration: number) {
        if (!this._clip_marker) this.create_clip_marker();

        this._clip_marker.x(start * this._marker_gap);
        this._clip_marker.width(duration * this._marker_gap);
    }


    timestamp_to_string(timestamp: number): string {
        return `${Math.floor(timestamp / 60)}:${(timestamp % 60)
            .toFixed(0)
            .padStart(2, "0")}`;
    }

    select_current_time(e: KonvaEventObject<WheelEvent>) {
        if (this._was_controlling_clip_marker) {
            this._clip_marker.stopDrag();
            this._was_controlling_clip_marker = false;
            return;
        }
        if (e.evt.button != 0) return;
        this._on_select_current_time(this._stage.getRelativePointerPosition().x / this._marker_gap);
    }

    start_clip_marker_change(e: KonvaEventObject<MouseEvent>) {
        if (e.evt.button != 0) return;

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
        if (!this._clip_marker) {
            this.create_clip_marker();
        }
    }

    move_clip_marker() {
        if (!this._mouse_drag_start) return;
        let range_end = this._stage.getRelativePointerPosition().x / this._marker_gap;
        let distance = Math.abs(this._mouse_drag_start - range_end);
        if (distance > 0.1) {
            this._clip_marker.x(
                (range_end < this._mouse_drag_start ? range_end : this._mouse_drag_start) *
                this._marker_gap
            );
            this._clip_marker.width(distance * this._marker_gap);
            this._was_controlling_clip_marker = true;
        }
    }

    complete_clip_marker_change() {
        if (!this._clip_marker) return;

        let duration = this._clip_marker.getSelfRect().width / this._marker_gap;
        let start = this._clip_marker.x() / this._marker_gap;

        this._mouse_drag_start = null;
        if (this._clip_marker.width() > 0) {
            clip_start.set(start);
            clip_end.set(start + duration);
        }
    }


    stage_horizontal_bound(x: number) {
        let right_bound =
            -(this._seconds_in_timeline * this._marker_gap * this._stage.scaleX()) +
            this._stage.width() -
            SCROLL_LEEWAY;
        return clamp(x, right_bound, SCROLL_LEEWAY);
    }

    stage_set_scale(scale: number, stage_position: number) {

        this._stage.scale({ x: scale, y: 1 });

        this._stage.position({
            x: stage_position,
            y: 0,
        });

        let childs = this._timeline_group.getChildren();
        for (let child of childs) {
            if (child.name() === CLIP_MARKER_NAME) continue;
            if (child.name().includes(THUMBNAIL_NAME)) continue;
            child.scaleX(1 / scale);
        }

        this.update_detail_visibility()
    }

    stage_zoom(e: KonvaEventObject<WheelEvent>) {
        e.evt.preventDefault();

        let old_scale = this._stage.scaleX();

        let pointer = this._stage.getPointerPosition();

        let mouse_point_to = {
            x: (pointer.x - this._stage.x()) / old_scale,
            y: (pointer.y - this._stage.y()) / old_scale,
        };

        let direction = e.evt.deltaY < 0 ? 1 : -1;

        let new_scale =
            direction > 0 ? old_scale * zoom_speed : old_scale / zoom_speed;

        if (new_scale <= 1) return;

        this.stage_set_scale(new_scale, pointer.x - mouse_point_to.x * new_scale);
    }

    update_stage_dimensions(width: number, height: number) {
        this._width = width;
        this._height = height;

        if (!this._stage) return;
        this._stage.width(width);
        this._stage.height(height);
    }



    update_current_time(current_time: number) {
        if (!this._current_time_marker) return;
        this._current_time_marker.x(current_time * this._marker_gap);
        this._current_time = current_time;
    }

    update_detail_visibility() {
        let details = this._timeline_group.getChildren((e) => e.name().includes(DETAIL_NAME))
        for (let detail of details) {
            detail.visible(this._stage.scaleX() > DETAIL_CUTOFF_SCALE);
        }
    }

    create_timeline_group(): Konva.Group {
        this._timeline_group = new Konva.Group();
        this._main_layer.add(this._timeline_group);
        return this._timeline_group;
    }

    create_clip_marker_transformer(): Konva.Transformer {
        this._clip_marker_transformer = new Konva.Transformer({
            name: CLIP_MARKER_NAME,
            nodes: [this._clip_marker],
            centeredScaling: false,
            rotateEnabled: false,
            enabledAnchors: ['middle-right'],
            boundBoxFunc: (oldBox, newBox) => {
                this._clip_marker.width(newBox.width / this._stage.scaleX());

                return oldBox;
            },
        });

        this._timeline_group.add(this._clip_marker_transformer);
        return this._clip_marker_transformer;
    }

    create_time_marker(timestamp: number): Konva.Rect {
        let mark = new Konva.Rect({
            name: TIME_MARKER_NAME,
            fill: "black",
            x: timestamp * this._marker_gap,
            y: 0,
            height: this._height,
            width: 1,
        });

        let text = new Konva.Text({
            name: DETAIL_NAME,
            fill: "black",
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

        this._timeline_group.add(mark);
        this._timeline_group.add(text);

        return mark;
    }

    create_clip_marker(): Konva.Rect {
        this._clip_marker = new Konva.Rect({
            name: CLIP_MARKER_NAME,
            fill: "blue",
            x: 0,
            y: this._height * 0.1,
            height: this._height * 0.5,
            width: 0,
            dragBoundFunc: function (pos) {
                return {
                    x: pos.x,
                    y: this.absolutePosition().y,
                };
            },
        });
        this._timeline_group.add(this._clip_marker);
        this.create_clip_marker_transformer();
        return this._clip_marker;
    }

    create_current_time_marker() {
        this._current_time_marker = new Konva.Rect({
            fill: "red",
            x: this._current_time * this._marker_gap,
            y: 0,
            height: this._height,
            width: 1
        });

        this._timeline_group.add(this._current_time_marker);

        return this._current_time_marker;
    }

    create_stage(): Konva.Stage {
        this._stage = new Konva.Stage({
            container: "stage",
            width: this._width,
            height: this._height,
            draggable: true,
            dragBoundFunc: (pos) => {
                return {
                    x: this.stage_horizontal_bound(pos.x),
                    y: this._stage.absolutePosition().y,
                };
            },
        });

        this._main_layer = new Konva.Layer();
        this._stage.add(this._main_layer);

        this._stage.on('wheel', this.stage_zoom.bind(this));
        this._stage.on('pointerclick', this.select_current_time.bind(this));
        this._stage.on('pointerdown', this.start_clip_marker_change.bind(this));
        this._stage.on('pointermove', this.move_clip_marker.bind(this));
        this._stage.on('pointerup', this.complete_clip_marker_change.bind(this));

        return this._stage;
    }

    async create_timeline_thumbnail(thumbnail_path: string, index: number) {
        let full_path = await join(thumbnail_path, `${index}.bmp`);

        console.log(index);
        Konva.Image.fromURL(convertFileSrc(full_path), (image) => {
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

    create_timeline_thumbnails(thumbnail_path: string, upto: number) {
        for (let i = this._created_thumbnails; i < upto; i++) {
            this.create_timeline_thumbnail(thumbnail_path, i);
        }
        this._created_thumbnails = upto;
    }
}