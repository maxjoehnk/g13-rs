use bitflags::bitflags;

bitflags! {
    pub struct Keys: u64 {
        const G1 = 1<<0;
        const G2 = 1<<1;
        const G3 = 1<<2;
        const G4 = 1<<3;
        const G5 = 1<<4;
        const G6 = 1<<5;
        const G7 = 1<<6;
        const G8 = 1<<7;
        const G9 = 1<<8;
        const G10 = 1<<9;
        const G11 = 1<<10;
        const G12 = 1<<11;
        const G13 = 1<<12;
        const G14 = 1<<13;
        const G15 = 1<<14;
        const G16 = 1<<15;
        const G17 = 1<<16;
        const G18 = 1<<17;
        const G19 = 1<<18;
        const G20 = 1<<19;
        const G21 = 1<<20;
        const G22 = 1<<21;

        const BD = 1<<24;
        const L1 = 1<<25;
        const L2 = 1<<26;
        const L3 = 1<<27;
        const L4 = 1<<28;

        const M1 = 1<<29;
        const M2 = 1<<30;
        const M3 = 1<<31;
        const MR = 1<<32;

        const LEFT = 1<<33;
        const DOWN = 1<<34;
        const JOYSTICK = 1<<35;
    }
}

bitflags! {
    pub struct ModeLeds: u8 {
        const M1 = 1<<0;
        const M2 = 1<<1;
        const M3 = 1<<2;
        const MR = 1<<3;
    }
}
