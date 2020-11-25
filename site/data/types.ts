
export enum CompilerMode {
    Check = 'Check',
    Debug = 'Debug',
    Release = 'Release',
}

export enum ProfileMode {
    Clean = 'Clean',
    Incremental = 'Incremental',
}
export enum System {
    TwoCores = '2 cores',
    FourCores = '4 cores',
    EightCores = '8 cores',
    SixteenCores = '16 cores',
}

export function getSystem(cores: number): System {
    switch (cores) {
        case 2: return System.TwoCores;
        case 4: return System.FourCores;
        case 8: return System.EightCores;
        case 16: return System.SixteenCores;
        default: return null;
    }
}