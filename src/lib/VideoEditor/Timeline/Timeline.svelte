<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import { clip_end, clip_start } from "../ClipOptions/ClipOptionsStore";
    import Konva from "konva";
    import type { KonvaEventObject } from "konva/lib/Node";
    export let video_current_time: number;
    export let onUpdate: (value: number) => void;

    Konva.dragButtons = [2];

    const clamp = (num, min, max) => Math.min(Math.max(num, min), max);

    let slider;

    let width: number;
    let height: number;

    $: console.dir(slider);

    export let seconds;
    const dispatch = createEventDispatcher();

    let current_time_marker: Konva.Rect;
    let marker_gap: number = 1;
    $: if (current_time_marker) {
        current_time_marker.x(video_current_time * marker_gap);
    }

    onMount(() => {
        console.log(seconds);

        marker_gap = width / seconds;

        const scroll_leeway = 50;
        let stage: Konva.Stage;

        let x_pos_clamp = (x) => {
            let right_bound =
                -(seconds * marker_gap * stage.scaleX()) +
                stage.width() -
                scroll_leeway;
            return clamp(x, right_bound, scroll_leeway);
        };

        stage = new Konva.Stage({
            container: "stage",
            width: width,
            height: height,
            draggable: true,
            dragBoundFunc: function (pos) {
                return {
                    x: x_pos_clamp(pos.x),
                    y: this.absolutePosition().y,
                };
            },
        });

        var layer = new Konva.Layer();
        stage.add(layer);

        let background = new Konva.Rect({
            fill: "red",
            x: 0,
            y: 0,
            width,
            height,
        });
        //layer.add(background);

        let timeline_group = new Konva.Group();
        layer.add(timeline_group);

        current_time_marker = new Konva.Rect({
            fill: "red",
            x: video_current_time * marker_gap,
            y: 0,
            height,
            width: 1,
            zIndex: 100,
        });

        let clip_range = new Konva.Rect({
            name: "clip_range",
            fill: "blue",
            x: 0,
            y: height * 0.1,
            height: height * 0.5,
            width: 0,
            dragBoundFunc:function (pos) {
                return {
                    x: pos.x,
                    y: this.absolutePosition().y,
                };
            },
        });

        let using_left_achor = false;

        var clip_range_tr = new Konva.Transformer({
            name: "clip_range",
            nodes: [clip_range],
            centeredScaling: false,
            rotateEnabled: false,
            enabledAnchors: ['middle-right'],
            boundBoxFunc(oldBox, newBox) {
                
                if(using_left_achor) {
                    clip_range.x(stage.getRelativePointerPosition().x)
                }else {
                    clip_range.width(newBox.width / stage.scaleX());
                }
               
                return oldBox;
            },
        });

        timeline_group.add(clip_range_tr);

        timeline_group.add(clip_range);

        timeline_group.add(current_time_marker);

        for (let i = 0; i < seconds; i++) {
            let mark = new Konva.Rect({
                fill: "black",
                x: i * marker_gap,
                y: 0,
                height,
                width: 1,
            });

            timeline_group.add(mark);
        }

        const scale_by = 1.1;
        var was_using_clip_range = false;

        stage.on("pointerclick", (e: KonvaEventObject<PointerEvent>) => {
            if (was_using_clip_range) {
                clip_range.stopDrag();
                was_using_clip_range = false;
                return;
            }
            if (e.evt.button != 0) return;
            onUpdate(stage.getRelativePointerPosition().x / marker_gap);
        });

        let range_start: number = null;

        stage.on("pointerdown", (e: KonvaEventObject<PointerEvent>) => {
            if (e.evt.button != 0) return;

            if(e.target.name() === "clip_range") {
                clip_range.startDrag();
                was_using_clip_range = true;
                return;
            }

            if(e.target.name().includes("_anchor")) {
                using_left_achor = e.target.name().includes("left") ;
                was_using_clip_range = true;
                return;
            }
            console.log(e);
            range_start = stage.getRelativePointerPosition().x / marker_gap;
        });

        stage.on("pointermove", (e) => {
            if (!range_start) return;
            let range_end = stage.getRelativePointerPosition().x / marker_gap;
            let distance = Math.abs(range_start - range_end);
            if (distance > 0.1) {
                clip_range.x(
                    (range_end < range_start ? range_end : range_start) *
                        marker_gap
                );
                clip_range.width(distance * marker_gap);
                was_using_clip_range = true;
            }
        });

        stage.on("pointerup", (e: KonvaEventObject<PointerEvent>) => {

            let duration = clip_range.getSelfRect().width / marker_gap;

            range_start = null;
            console.log(clip_range.width());
            if(clip_range.width() > 0) {
                $clip_start = clip_range.x();
                $clip_end = clip_range.x() + duration;
            }
        });

        stage.on("wheel", (e) => {
            e.evt.preventDefault();

            let old_scale = stage.scaleX();

            let pointer = stage.getPointerPosition();

            let mouse_point_to = {
                x: (pointer.x - stage.x()) / old_scale,
                y: (pointer.y - stage.y()) / old_scale,
            };

            let direction = e.evt.deltaY < 0 ? 1 : -1;

            let new_scale =
                direction > 0 ? old_scale * scale_by : old_scale / scale_by;

            if (new_scale <= 1) return;

            stage.scale({ x: new_scale, y: 1 });

            stage.position({
                x: x_pos_clamp(pointer.x - mouse_point_to.x * new_scale),
                y: 0,
            });

            let childs = timeline_group.getChildren();
            for (let child of childs) {
                if (child.name() === "clip_range") continue;
                child.scaleX(1 / new_scale);
            }
        });
    });
</script>

<div class="h-16" bind:clientWidth={width} bind:clientHeight={height}>
    <div id="stage" on:contextmenu={(e) => e.preventDefault()} />
</div>
