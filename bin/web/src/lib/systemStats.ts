export interface SystemStats {
    disk_stats: DiskStats;
    memory_stats: MemoryStats;
}

export interface DiskStats {
    partition: string,
    total_size: string,
    used_size: string,
    available_size: string,
    used_percent: string,
    mounted_on: string,
}

export interface MemoryStats {
    total: string,
    used: string,
    free: string,
}
