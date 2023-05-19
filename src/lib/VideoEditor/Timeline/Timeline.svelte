<script lang="ts">
    export let seconds;
    let scaleFactor = 1.1;

    type CTX = CanvasRenderingContext2D & {
        transformedPoint: (a: number, b: number) => DOMPoint;
    };

    var lastX: number;
    var lastY: number;

    let dragStart, dragged: boolean;

    let canvas: HTMLCanvasElement;
    let ctx: CTX;

    function draw_marker(index: number) {
        ctx.fillStyle = "black";
        ctx.fillRect(2 * index, 0, 1, ctx.canvas.height);
    }

    function redraw() {
        var p1 = ctx.transformedPoint(0, 0);
        var p2 = ctx.transformedPoint(canvas.width, canvas.height);
        ctx.clearRect(p1.x, p1.y, p2.x - p1.x, p2.y - p1.y);

        ctx.fillStyle = "rgb(200, 0,0)";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        for (let i = 0; i < seconds; i++) {
            draw_marker(i);
        }
    }

    $: if (canvas) {
        (lastX = canvas.width / 2), (lastY = canvas.height / 2);

        ctx = canvas.getContext("2d") as CTX;
        track_transforms(ctx);
        redraw();
    }

    function track_transforms(ctx: CTX) {
		var svg = document.createElementNS("http://www.w3.org/2000/svg",'svg');
		var xform = svg.createSVGMatrix();
		ctx.getTransform = function(){ return xform; };
		
		var savedTransforms = [];
		var save = ctx.save;
		ctx.save = function(){
			savedTransforms.push(xform.translate(0,0));
			return save.call(ctx);
		};
		var restore = ctx.restore;
		ctx.restore = function(){
			xform = savedTransforms.pop();
			return restore.call(ctx);
		};

		var scale = ctx.scale;
		ctx.scale = function(sx,sy){
			xform = xform.scale(sx,sy);
			return scale.call(ctx,sx,sy);
		};
		var rotate = ctx.rotate;
		ctx.rotate = function(radians){
			xform = xform.rotate(radians*180/Math.PI);
			return rotate.call(ctx,radians);
		};
		var translate = ctx.translate;
		ctx.translate = function(dx,dy){
			xform = xform.translate(dx,dy);
			return translate.call(ctx,dx,dy);
		};
		var transform = ctx.transform;
		ctx.transform = function(a,b,c,d,e,f){
			var m2 = svg.createSVGMatrix();
			m2.a=a; m2.b=b; m2.c=c; m2.d=d; m2.e=e; m2.f=f;
			xform = xform.multiply(m2);
			return transform.call(ctx,a,b,c,d,e,f);
		};
		var setTransform = ctx.setTransform;
		ctx.setTransform = function(a,b,c,d,e,f){
			xform.a = a;
			xform.b = b;
			xform.c = c;
			xform.d = d;
			xform.e = e;
			xform.f = f;
			return setTransform.call(ctx,a,b,c,d,e,f);
		} as any;
		var pt  = svg.createSVGPoint();
		ctx.transformedPoint = function(x,y){
			pt.x=x; pt.y=y;
			return pt.matrixTransform(xform.inverse());
		}
	}

    function zoom(delta: number) {
        var pt = ctx.transformedPoint(lastX, lastY);
        ctx.translate(pt.x, pt.y);
        var factor = Math.pow(scaleFactor, delta);
        ctx.scale(factor, factor);
        ctx.translate(-pt.x, -pt.y);
        redraw();
    }
    function mouse_down(e: MouseEvent) {
        lastX = e.offsetX || e.pageX - canvas.offsetLeft;
        lastY = e.offsetY || e.pageY - canvas.offsetTop;
        dragStart = ctx.transformedPoint(lastX, lastY);
        dragged = false;
    }

    function mouse_up(e: MouseEvent) {
        dragStart = null;
    }

    function mouse_move(e: MouseEvent) {
        lastX = e.offsetX || e.pageX - canvas.offsetLeft;
        lastY = e.offsetY || e.pageY - canvas.offsetTop;
        dragged = true;
        if (dragStart) {
            var pt = ctx.transformedPoint(lastX, lastY);
            ctx.translate(pt.x - dragStart.x, pt.y - dragStart.y);
            redraw();
        }
    }

    function on_scroll(e: WheelEvent) {
        zoom(-e.deltaY / 100);
    }
</script>

<canvas
    on:mouseup={mouse_up}
    on:mousemove={mouse_move}
    on:mousedown={mouse_down}
    on:wheel={(e) => on_scroll(e)}
    bind:this={canvas}
    class="h-24 w-5/12"
/>
