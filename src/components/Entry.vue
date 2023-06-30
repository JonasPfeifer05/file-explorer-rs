<script lang="ts">
import {EntryDescriptor} from "../objects/EntryDescriptor"
import {defineComponent, PropType, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

export default defineComponent({
    name: "Entry",
    emits: ["selected"],
    props: {
        entry: {
            type: Object as PropType<EntryDescriptor>,
            required: true,
        },
        path: {
            type: String,
        },
    },
    methods: {
        emitName: function (event: Event) {
            this.$emit('selected', this.entry.name)
        },
        calculateSize: function (event: Event) {
            event.stopPropagation();
            if (this.entry.size !== -1) return;
            this.entry.size = -2;
            invoke("dirSize", { path: this.path+"\\"+this.entry.name } ).then(value => {
                this.entry.size = value as number;
            })
        }
    },
    computed: {
        size: function (): string {
            if (this.entry.size >= 1_000_000_000) {
                return round(this.entry.size/1_000_000_000, 2) + " Gb";
            } else if (this.entry.size >= 1_000_000) {
                return round(this.entry.size/1_000_000,2) + "Mb";
            } else if (this.entry.size >= 1_000) {
                return round(this.entry.size/1_000,2) + " Kb";
            } else {
                return this.entry.size + " B"
            }
        },
        calculationState: function (): string {
            if (this.entry.size == -1) return "Compute Size";
            if (this.entry.size == -2) return "Calculating...";
            return "";
        }
    }
})

function round(num: number, precision: number): number {
    return Math.round(num * Math.pow(10, precision)) / Math.pow(10, precision);
}
</script>

<template>
    <div @click="emitName($event)" class="entry">
        <img v-if="entry.entryType === 'file'" src="../assets/file.png" alt="image of the entry" class="entryImg">
        <img v-else src="../assets/folder.png" alt="image of the entry" class="entryImg">
        <span class="entryText">
            <span>
                {{ entry.name }}
            </span>
            <span v-if="entry.size>=0">
                {{ size }}
            </span>
            <button v-else class="calculateSizeBtn" @click="calculateSize($event)">{{ calculationState }}</button>
        </span>
    </div>
</template>

<style scoped>
.entry {
    cursor: pointer;
    display: flex;
    align-items: center;

    gap: 10px;

    height: 40px;
}

.entryImg {
    max-height: 20px;
    max-width: 20px;
}

.entryText {
    display: flex;
    justify-content: space-between;
    align-items: center;

    width: 100%;

    font-size: medium;
}

.calculateSizeBtn {
    cursor: pointer;

    background: #4c4c4c;
    border: none;
    height: 25px;
    color: lightgray;
    border-radius: 3px;
    transition-duration: 250ms;
}
.calculateSizeBtn:hover {
    background: #666;
}
</style>