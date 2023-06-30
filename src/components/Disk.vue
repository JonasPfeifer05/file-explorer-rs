<script lang="ts">
import {DiskDescriptor} from "../objects/DiskDescriptor";
import {defineComponent, PropType} from "vue";

export default defineComponent({
    name: "Disk",
    emits: ["selected"],
    props: {
        disk: {
            type: Object as PropType<DiskDescriptor>,
            required: true,
        }
    },
    methods: {
        emitName: function () {
            this.$emit('selected', this.disk.name)
        }
    }
})
</script>

<template>
    <div @click="emitName" class="disk">
        <img src="../assets/disk.png" alt="disk" class="diskImg">
        <span class="diskName">
            {{ disk.name.substring(0, disk.name.length-2) }}
        </span>
    </div>
</template>

<style scoped>
.disk {
    cursor: pointer;
    display: flex;
    align-items: center;

    gap: 20px;

    height: 50px;
}

.diskImg {
    max-height: 35px;
    max-width: 35px;
}

.diskName {
    font-size: large;
}
</style>