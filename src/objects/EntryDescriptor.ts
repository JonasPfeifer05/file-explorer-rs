export interface EntryDescriptor {
    entryType: "file"|"directory",
    name: string,
    size: number,
}