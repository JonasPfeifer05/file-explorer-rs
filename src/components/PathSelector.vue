<script lang="ts">
import {defineComponent, ref} from 'vue'
import {PropType} from "vue/dist/vue";

export default defineComponent({
    name: "PathSelector",
    emits: ["pathChanged"],
    props: {
        path: {
            type: Object as PropType<string[]>,
            required: true,
        },
    },
    methods: {
        moveOneBack: function () {
            this.path.pop();
            this.$emit("pathChanged");
        },
        shortenPath: function (newLength: number) {
            this.path.length = newLength;
            this.$emit("pathChanged");
        },
        startScroll: function (event: MouseEvent) {
            this.scrolling = true;
            this.scrollPosition = [event.screenX, event.screenY];
            console.log("start")
        },
        stopScroll: function () {
            this.scrolling = false;
            console.log("stop")
        },
        scrollMoved: function (event: MouseEvent) {
            if (!this.scrolling) return;

            let dx = event.screenX - this.scrollPosition[0];
            let dy = event.screenY - this.scrollPosition[1];

            (this.$refs.elements as HTMLElement).scrollBy(-dx, 0);

            this.scrollPosition = [event.screenX, event.screenY];

            console.log(dx, dy)
        }
    },
    data: function () {
        return {
            scrolling: false,
            scrollPosition: [0, 0],
        };
    },
})
</script>

<template>
    <div id="header">
        <div id="backHolder" @click="moveOneBack">
            <i id="back" class="arrow left"></i>
        </div>
        <div id="elements" @mousedown="startScroll($event)" @mouseleave="stopScroll" @mouseup="stopScroll" @mousemove="scrollMoved($event)" ref="elements" :class="{dragging: scrolling}">
            <div class="element" @click="shortenPath(0)">Root</div>
            <template v-for="(element, index) in path">
                <i class="arrow right pathSpacer"></i>
                <div class="element" @click="shortenPath(index+1)">{{ element }}</div>
            </template>
        </div>
    </div>
</template>

<style scoped>
.dragging{
    cursor: grabbing !important;
    user-select: none;
    -moz-user-select: none;
    -webkit-text-select: none;
    -webkit-user-select: none;
}

#header {
    display: flex;
    align-items: center;

    gap: 20px;

    height: 60px;

    box-sizing: border-box;
    padding: 20px;
    margin: 0;

    background: #2c2c2c;
}

#back {
    width: 5px;
    height: 5px;

    translate: calc(5px * 0.7071) 0 0;
}

#elements {
    display: flex;
    align-items: center;

    gap: 10px;

    width: calc(100% - 65px);
    height: 40px;

    margin: 0;
    padding: 3px;

    box-sizing: border-box;

    border: #535353 1.5px solid;

    line-height: calc(40px - 10px - 3px);
    vertical-align: middle;

    overflow-y: hidden;
    overflow-x: scroll;

    cursor: grab;
}

#elements::-webkit-scrollbar {
    display: none;
}

.arrow {
    border: solid white;
    border-width: 0 2px 2px 0;
    display: inline-block;
    padding: 3px;
}

.left {
    transform: rotate(135deg);
}

.right {
    transform: rotate(-45deg);
}

.pathSpacer {
    width: 2px !important;
    min-width: 2px !important;
    height: 2px !important;
    min-height: 2px !important;

    translate: calc(2px * -0.7072) 0 0;
}

.element {
    cursor: pointer;

    display: flex;
    align-items: center;

    height: 100%;

    background: #434343;

    box-sizing: border-box;
    padding: 0 10px;

    border-radius: 2px;
    white-space: nowrap;

    transition-duration: 250ms;
}

.element:hover {
    background: #383838;
}

#backHolder {
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;

    width: 25px;
    min-width: 25px;
    height: 25px;
    min-height: 25px;

    border-radius: 5px;

    transition-duration: 250ms;
}

#backHolder:hover {
    background: #383838;
}
</style>