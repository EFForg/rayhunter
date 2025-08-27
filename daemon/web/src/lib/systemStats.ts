export interface SystemStats {
    disk_stats: DiskStats;
    memory_stats: MemoryStats;
    runtime_metadata: RuntimeMetadata;
    battery_status?: BatteryStatus;
}

export interface RuntimeMetadata {
    rayhunter_version: string;
    system_os: string;
    arch: string;
}

export interface DiskStats {
    partition: string;
    total_size: string;
    used_size: string;
    available_size: string;
    used_percent: string;
    mounted_on: string;
}

export interface MemoryStats {
    total: string;
    used: string;
    free: string;
}

export interface BatteryStatus {
    level: BatteryLevel;
    is_plugged_in: boolean;
}

export enum BatteryLevel {
    VeryLow = 'VeryLow',
    Low = 'Low',
    Medium = 'Medium',
    High = 'High',
    Full = 'Full',
}
