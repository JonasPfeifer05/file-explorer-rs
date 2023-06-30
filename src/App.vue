<script lang="ts">
import {defineComponent, ref} from 'vue'
import Disk from "./components/Disk.vue";
import Entry from "./components/Entry.vue";
import {DiskDescriptor} from "./objects/DiskDescriptor";
import {EntryDescriptor} from "./objects/EntryDescriptor";
import {invoke} from "@tauri-apps/api/tauri";
import PathSelector from "./components/PathSelector.vue";

export default defineComponent({
    name: "App",
    components: {PathSelector, Disk, Entry},
    setup() {
        let data = ref({
            path: [] as string[],
            disks: [] as DiskDescriptor[],
            entries: [] as EntryDescriptor[]
        });

        invoke("disks").then(value => {
            data.value.disks = value as DiskDescriptor[];
        })

        return data.value;
    },
    methods: {
        selectDisk: function (disk: string) {
            this.path.push(disk);
            this.updateEntries();
        },
        updateEntries: function () {
            invoke("entries", {path: this.getPathAsString}).then(value => {
                this.entries = value as EntryDescriptor[];
            })
        },
        moveIntoDirectory: function (name: string) {
            this.path.push(name);
            this.updateEntries();
        },
    },
    computed: {
        getPathAsString: function (): string {
            return this.path.join("\\");
        }
    }
})

document.addEventListener("DOMContentLoaded", () => {
    // This will wait for the window to load, but you could
    // run this function on whatever trigger you want
    invoke("show_main");
});
</script>

<template>
    <div id="app-content">
        <path-selector :path="path" @path-changed="updateEntries"></path-selector>
        <div id="entryList">
            <Disk v-if="path.length===0" v-for="disk in disks" :disk="disk"
                  @selected="(name) => selectDisk(name)"></Disk>
            <Entry v-else v-for="entry in entries" :entry="entry" :path="getPathAsString" @selected="(name) => moveIntoDirectory(name)"></Entry>
        </div>
    </div>
</template>

<style scoped>
#app-content {
    width: 100vw;
    height: 100vh;
    max-height: 100vh;

    overflow: hidden;

    background: #191919;
}

#entryList {
    height: calc(100vh - 60px);

    padding: 10px 20px 10px;
    overflow: scroll;

    scrollbar-color: #2c2c2c #2c2c2c;
}

/* width */
#entryList::-webkit-scrollbar {
    width: 10px;
}

/* Track */
#entryList::-webkit-scrollbar-track {
    background: #2c2c2c;
}

/* Handle */
#entryList::-webkit-scrollbar-thumb {
    background: #424242 ;
    border-radius: 10px;
}

/* Handle on hover */
#entryList::-webkit-scrollbar-thumb:hover {
    background: #535353;
}
</style>