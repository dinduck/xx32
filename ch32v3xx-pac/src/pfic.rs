#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    pub isr1: ISR1,
    #[doc = "0x04 - Interrupt Status Register"]
    pub isr2: ISR2,
    #[doc = "0x08 - Interrupt Status Register"]
    pub isr3: ISR3,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr4: ISR4,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Interrupt Pending Register"]
    pub ipr1: IPR1,
    #[doc = "0x24 - Interrupt Pending Register"]
    pub ipr2: IPR2,
    #[doc = "0x28 - Interrupt Pending Register"]
    pub ipr3: IPR3,
    #[doc = "0x2c - Interrupt Pending Register"]
    pub ipr4: IPR4,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - Interrupt Priority Register"]
    pub ithresdr: ITHRESDR,
    _reserved9: [u8; 0x04],
    #[doc = "0x48 - Interrupt Config Register"]
    pub cfgr: CFGR,
    #[doc = "0x4c - Interrupt Global Register"]
    pub gisr: GISR,
    #[doc = "0x50 - ID Config Register"]
    pub vtfidr: VTFIDR,
    _reserved12: [u8; 0x0c],
    #[doc = "0x60 - Interrupt 0 address Register"]
    pub vtfaddrr0: VTFADDRR0,
    #[doc = "0x64 - Interrupt 1 address Register"]
    pub vtfaddrr1: VTFADDRR1,
    #[doc = "0x68 - Interrupt 2 address Register"]
    pub vtfaddrr2: VTFADDRR2,
    #[doc = "0x6c - Interrupt 3 address Register"]
    pub vtfaddrr3: VTFADDRR3,
    _reserved16: [u8; 0x90],
    #[doc = "0x100 - Interrupt Setting Register"]
    pub ienr1: IENR1,
    #[doc = "0x104 - Interrupt Setting Register"]
    pub ienr2: IENR2,
    #[doc = "0x108 - Interrupt Setting Register"]
    pub ienr3: IENR3,
    #[doc = "0x10c - Interrupt Setting Register"]
    pub ienr4: IENR4,
    _reserved20: [u8; 0x70],
    #[doc = "0x180 - Interrupt Clear Register"]
    pub irer1: IRER1,
    #[doc = "0x184 - Interrupt Clear Register"]
    pub irer2: IRER2,
    #[doc = "0x188 - Interrupt Clear Register"]
    pub irer3: IRER3,
    #[doc = "0x18c - Interrupt Clear Register"]
    pub irer4: IRER4,
    _reserved24: [u8; 0x70],
    #[doc = "0x200 - Interrupt Pending Register"]
    pub ipsr1: IPSR1,
    #[doc = "0x204 - Interrupt Pending Register"]
    pub ipsr2: IPSR2,
    #[doc = "0x208 - Interrupt Pending Register"]
    pub ipsr3: IPSR3,
    #[doc = "0x20c - Interrupt Pending Register"]
    pub ipsr4: IPSR4,
    _reserved28: [u8; 0x70],
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    pub iprr1: IPRR1,
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    pub iprr2: IPRR2,
    #[doc = "0x288 - Interrupt Pending Clear Register"]
    pub iprr3: IPRR3,
    #[doc = "0x28c - Interrupt Pending Clear Register"]
    pub iprr4: IPRR4,
    _reserved32: [u8; 0x70],
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    pub iactr1: IACTR1,
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    pub iactr2: IACTR2,
    #[doc = "0x308 - Interrupt ACTIVE Register"]
    pub iactr3: IACTR3,
    #[doc = "0x30c - Interrupt ACTIVE Register"]
    pub iactr4: IACTR4,
    _reserved36: [u8; 0xf0],
    #[doc = "0x400 - Interrupt Priority Register"]
    pub iprior0: IPRIOR0,
    #[doc = "0x401 - Interrupt Priority Register"]
    pub iprior1: IPRIOR1,
    #[doc = "0x402 - Interrupt Priority Register"]
    pub iprior2: IPRIOR2,
    #[doc = "0x403 - Interrupt Priority Register"]
    pub iprior3: IPRIOR3,
    #[doc = "0x404 - Interrupt Priority Register"]
    pub iprior4: IPRIOR4,
    #[doc = "0x405 - Interrupt Priority Register"]
    pub iprior5: IPRIOR5,
    #[doc = "0x406 - Interrupt Priority Register"]
    pub iprior6: IPRIOR6,
    #[doc = "0x407 - Interrupt Priority Register"]
    pub iprior7: IPRIOR7,
    #[doc = "0x408 - Interrupt Priority Register"]
    pub iprior8: IPRIOR8,
    #[doc = "0x409 - Interrupt Priority Register"]
    pub iprior9: IPRIOR9,
    #[doc = "0x40a - Interrupt Priority Register"]
    pub iprior10: IPRIOR10,
    #[doc = "0x40b - Interrupt Priority Register"]
    pub iprior11: IPRIOR11,
    #[doc = "0x40c - Interrupt Priority Register"]
    pub iprior12: IPRIOR12,
    #[doc = "0x40d - Interrupt Priority Register"]
    pub iprior13: IPRIOR13,
    #[doc = "0x40e - Interrupt Priority Register"]
    pub iprior14: IPRIOR14,
    #[doc = "0x40f - Interrupt Priority Register"]
    pub iprior15: IPRIOR15,
    #[doc = "0x410 - Interrupt Priority Register"]
    pub iprior16: IPRIOR16,
    #[doc = "0x411 - Interrupt Priority Register"]
    pub iprior17: IPRIOR17,
    #[doc = "0x412 - Interrupt Priority Register"]
    pub iprior18: IPRIOR18,
    #[doc = "0x413 - Interrupt Priority Register"]
    pub iprior19: IPRIOR19,
    #[doc = "0x414 - Interrupt Priority Register"]
    pub iprior20: IPRIOR20,
    #[doc = "0x415 - Interrupt Priority Register"]
    pub iprior21: IPRIOR21,
    #[doc = "0x416 - Interrupt Priority Register"]
    pub iprior22: IPRIOR22,
    #[doc = "0x417 - Interrupt Priority Register"]
    pub iprior23: IPRIOR23,
    #[doc = "0x418 - Interrupt Priority Register"]
    pub iprior24: IPRIOR24,
    #[doc = "0x419 - Interrupt Priority Register"]
    pub iprior25: IPRIOR25,
    #[doc = "0x41a - Interrupt Priority Register"]
    pub iprior26: IPRIOR26,
    #[doc = "0x41b - Interrupt Priority Register"]
    pub iprior27: IPRIOR27,
    #[doc = "0x41c - Interrupt Priority Register"]
    pub iprior28: IPRIOR28,
    #[doc = "0x41d - Interrupt Priority Register"]
    pub iprior29: IPRIOR29,
    #[doc = "0x41e - Interrupt Priority Register"]
    pub iprior30: IPRIOR30,
    #[doc = "0x41f - Interrupt Priority Register"]
    pub iprior31: IPRIOR31,
    #[doc = "0x420 - Interrupt Priority Register"]
    pub iprior32: IPRIOR32,
    #[doc = "0x421 - Interrupt Priority Register"]
    pub iprior33: IPRIOR33,
    #[doc = "0x422 - Interrupt Priority Register"]
    pub iprior34: IPRIOR34,
    #[doc = "0x423 - Interrupt Priority Register"]
    pub iprior35: IPRIOR35,
    #[doc = "0x424 - Interrupt Priority Register"]
    pub iprior36: IPRIOR36,
    #[doc = "0x425 - Interrupt Priority Register"]
    pub iprior37: IPRIOR37,
    #[doc = "0x426 - Interrupt Priority Register"]
    pub iprior38: IPRIOR38,
    #[doc = "0x427 - Interrupt Priority Register"]
    pub iprior39: IPRIOR39,
    #[doc = "0x428 - Interrupt Priority Register"]
    pub iprior40: IPRIOR40,
    #[doc = "0x429 - Interrupt Priority Register"]
    pub iprior41: IPRIOR41,
    #[doc = "0x42a - Interrupt Priority Register"]
    pub iprior42: IPRIOR42,
    #[doc = "0x42b - Interrupt Priority Register"]
    pub iprior43: IPRIOR43,
    #[doc = "0x42c - Interrupt Priority Register"]
    pub iprior44: IPRIOR44,
    #[doc = "0x42d - Interrupt Priority Register"]
    pub iprior45: IPRIOR45,
    #[doc = "0x42e - Interrupt Priority Register"]
    pub iprior46: IPRIOR46,
    #[doc = "0x42f - Interrupt Priority Register"]
    pub iprior47: IPRIOR47,
    #[doc = "0x430 - Interrupt Priority Register"]
    pub iprior48: IPRIOR48,
    #[doc = "0x431 - Interrupt Priority Register"]
    pub iprior49: IPRIOR49,
    #[doc = "0x432 - Interrupt Priority Register"]
    pub iprior50: IPRIOR50,
    #[doc = "0x433 - Interrupt Priority Register"]
    pub iprior51: IPRIOR51,
    #[doc = "0x434 - Interrupt Priority Register"]
    pub iprior52: IPRIOR52,
    #[doc = "0x435 - Interrupt Priority Register"]
    pub iprior53: IPRIOR53,
    #[doc = "0x436 - Interrupt Priority Register"]
    pub iprior54: IPRIOR54,
    #[doc = "0x437 - Interrupt Priority Register"]
    pub iprior55: IPRIOR55,
    #[doc = "0x438 - Interrupt Priority Register"]
    pub iprior56: IPRIOR56,
    #[doc = "0x439 - Interrupt Priority Register"]
    pub iprior57: IPRIOR57,
    #[doc = "0x43a - Interrupt Priority Register"]
    pub iprior58: IPRIOR58,
    #[doc = "0x43b - Interrupt Priority Register"]
    pub iprior59: IPRIOR59,
    #[doc = "0x43c - Interrupt Priority Register"]
    pub iprior60: IPRIOR60,
    #[doc = "0x43d - Interrupt Priority Register"]
    pub iprior61: IPRIOR61,
    #[doc = "0x43e - Interrupt Priority Register"]
    pub iprior62: IPRIOR62,
    #[doc = "0x43f - Interrupt Priority Register"]
    pub iprior63: IPRIOR63,
    #[doc = "0x440 - Interrupt Priority Register"]
    pub iprior64: IPRIOR64,
    #[doc = "0x441 - Interrupt Priority Register"]
    pub iprior65: IPRIOR65,
    #[doc = "0x442 - Interrupt Priority Register"]
    pub iprior66: IPRIOR66,
    #[doc = "0x443 - Interrupt Priority Register"]
    pub iprior67: IPRIOR67,
    #[doc = "0x444 - Interrupt Priority Register"]
    pub iprior68: IPRIOR68,
    #[doc = "0x445 - Interrupt Priority Register"]
    pub iprior69: IPRIOR69,
    #[doc = "0x446 - Interrupt Priority Register"]
    pub iprior70: IPRIOR70,
    #[doc = "0x447 - Interrupt Priority Register"]
    pub iprior71: IPRIOR71,
    #[doc = "0x448 - Interrupt Priority Register"]
    pub iprior72: IPRIOR72,
    #[doc = "0x449 - Interrupt Priority Register"]
    pub iprior73: IPRIOR73,
    #[doc = "0x44a - Interrupt Priority Register"]
    pub iprior74: IPRIOR74,
    #[doc = "0x44b - Interrupt Priority Register"]
    pub iprior75: IPRIOR75,
    #[doc = "0x44c - Interrupt Priority Register"]
    pub iprior76: IPRIOR76,
    #[doc = "0x44d - Interrupt Priority Register"]
    pub iprior77: IPRIOR77,
    #[doc = "0x44e - Interrupt Priority Register"]
    pub iprior78: IPRIOR78,
    #[doc = "0x44f - Interrupt Priority Register"]
    pub iprior79: IPRIOR79,
    #[doc = "0x450 - Interrupt Priority Register"]
    pub iprior80: IPRIOR80,
    #[doc = "0x451 - Interrupt Priority Register"]
    pub iprior81: IPRIOR81,
    #[doc = "0x452 - Interrupt Priority Register"]
    pub iprior82: IPRIOR82,
    #[doc = "0x453 - Interrupt Priority Register"]
    pub iprior83: IPRIOR83,
    #[doc = "0x454 - Interrupt Priority Register"]
    pub iprior84: IPRIOR84,
    #[doc = "0x455 - Interrupt Priority Register"]
    pub iprior85: IPRIOR85,
    #[doc = "0x456 - Interrupt Priority Register"]
    pub iprior86: IPRIOR86,
    #[doc = "0x457 - Interrupt Priority Register"]
    pub iprior87: IPRIOR87,
    #[doc = "0x458 - Interrupt Priority Register"]
    pub iprior88: IPRIOR88,
    #[doc = "0x459 - Interrupt Priority Register"]
    pub iprior89: IPRIOR89,
    #[doc = "0x45a - Interrupt Priority Register"]
    pub iprior90: IPRIOR90,
    #[doc = "0x45b - Interrupt Priority Register"]
    pub iprior91: IPRIOR91,
    #[doc = "0x45c - Interrupt Priority Register"]
    pub iprior92: IPRIOR92,
    #[doc = "0x45d - Interrupt Priority Register"]
    pub iprior93: IPRIOR93,
    #[doc = "0x45e - Interrupt Priority Register"]
    pub iprior94: IPRIOR94,
    #[doc = "0x45f - Interrupt Priority Register"]
    pub iprior95: IPRIOR95,
    #[doc = "0x460 - Interrupt Priority Register"]
    pub iprior96: IPRIOR96,
    #[doc = "0x461 - Interrupt Priority Register"]
    pub iprior97: IPRIOR97,
    #[doc = "0x462 - Interrupt Priority Register"]
    pub iprior98: IPRIOR98,
    #[doc = "0x463 - Interrupt Priority Register"]
    pub iprior99: IPRIOR99,
    #[doc = "0x464 - Interrupt Priority Register"]
    pub iprior100: IPRIOR100,
    #[doc = "0x465 - Interrupt Priority Register"]
    pub iprior101: IPRIOR101,
    #[doc = "0x466 - Interrupt Priority Register"]
    pub iprior102: IPRIOR102,
    #[doc = "0x467 - Interrupt Priority Register"]
    pub iprior103: IPRIOR103,
    #[doc = "0x468 - Interrupt Priority Register"]
    pub iprior104: IPRIOR104,
    #[doc = "0x469 - Interrupt Priority Register"]
    pub iprior105: IPRIOR105,
    #[doc = "0x46a - Interrupt Priority Register"]
    pub iprior106: IPRIOR106,
    #[doc = "0x46b - Interrupt Priority Register"]
    pub iprior107: IPRIOR107,
    #[doc = "0x46c - Interrupt Priority Register"]
    pub iprior108: IPRIOR108,
    #[doc = "0x46d - Interrupt Priority Register"]
    pub iprior109: IPRIOR109,
    #[doc = "0x46e - Interrupt Priority Register"]
    pub iprior110: IPRIOR110,
    #[doc = "0x46f - Interrupt Priority Register"]
    pub iprior111: IPRIOR111,
    #[doc = "0x470 - Interrupt Priority Register"]
    pub iprior112: IPRIOR112,
    #[doc = "0x471 - Interrupt Priority Register"]
    pub iprior113: IPRIOR113,
    #[doc = "0x472 - Interrupt Priority Register"]
    pub iprior114: IPRIOR114,
    #[doc = "0x473 - Interrupt Priority Register"]
    pub iprior115: IPRIOR115,
    #[doc = "0x474 - Interrupt Priority Register"]
    pub iprior116: IPRIOR116,
    #[doc = "0x475 - Interrupt Priority Register"]
    pub iprior117: IPRIOR117,
    #[doc = "0x476 - Interrupt Priority Register"]
    pub iprior118: IPRIOR118,
    #[doc = "0x477 - Interrupt Priority Register"]
    pub iprior119: IPRIOR119,
    #[doc = "0x478 - Interrupt Priority Register"]
    pub iprior120: IPRIOR120,
    #[doc = "0x479 - Interrupt Priority Register"]
    pub iprior121: IPRIOR121,
    #[doc = "0x47a - Interrupt Priority Register"]
    pub iprior122: IPRIOR122,
    #[doc = "0x47b - Interrupt Priority Register"]
    pub iprior123: IPRIOR123,
    #[doc = "0x47c - Interrupt Priority Register"]
    pub iprior124: IPRIOR124,
    #[doc = "0x47d - Interrupt Priority Register"]
    pub iprior125: IPRIOR125,
    #[doc = "0x47e - Interrupt Priority Register"]
    pub iprior126: IPRIOR126,
    #[doc = "0x47f - Interrupt Priority Register"]
    pub iprior127: IPRIOR127,
    #[doc = "0x480 - Interrupt Priority Register"]
    pub iprior128: IPRIOR128,
    #[doc = "0x481 - Interrupt Priority Register"]
    pub iprior129: IPRIOR129,
    #[doc = "0x482 - Interrupt Priority Register"]
    pub iprior130: IPRIOR130,
    #[doc = "0x483 - Interrupt Priority Register"]
    pub iprior131: IPRIOR131,
    #[doc = "0x484 - Interrupt Priority Register"]
    pub iprior132: IPRIOR132,
    #[doc = "0x485 - Interrupt Priority Register"]
    pub iprior133: IPRIOR133,
    #[doc = "0x486 - Interrupt Priority Register"]
    pub iprior134: IPRIOR134,
    #[doc = "0x487 - Interrupt Priority Register"]
    pub iprior135: IPRIOR135,
    #[doc = "0x488 - Interrupt Priority Register"]
    pub iprior136: IPRIOR136,
    #[doc = "0x489 - Interrupt Priority Register"]
    pub iprior137: IPRIOR137,
    #[doc = "0x48a - Interrupt Priority Register"]
    pub iprior138: IPRIOR138,
    #[doc = "0x48b - Interrupt Priority Register"]
    pub iprior139: IPRIOR139,
    #[doc = "0x48c - Interrupt Priority Register"]
    pub iprior140: IPRIOR140,
    #[doc = "0x48d - Interrupt Priority Register"]
    pub iprior141: IPRIOR141,
    #[doc = "0x48e - Interrupt Priority Register"]
    pub iprior142: IPRIOR142,
    #[doc = "0x48f - Interrupt Priority Register"]
    pub iprior143: IPRIOR143,
    #[doc = "0x490 - Interrupt Priority Register"]
    pub iprior144: IPRIOR144,
    #[doc = "0x491 - Interrupt Priority Register"]
    pub iprior145: IPRIOR145,
    #[doc = "0x492 - Interrupt Priority Register"]
    pub iprior146: IPRIOR146,
    #[doc = "0x493 - Interrupt Priority Register"]
    pub iprior147: IPRIOR147,
    #[doc = "0x494 - Interrupt Priority Register"]
    pub iprior148: IPRIOR148,
    #[doc = "0x495 - Interrupt Priority Register"]
    pub iprior149: IPRIOR149,
    #[doc = "0x496 - Interrupt Priority Register"]
    pub iprior150: IPRIOR150,
    #[doc = "0x497 - Interrupt Priority Register"]
    pub iprior151: IPRIOR151,
    #[doc = "0x498 - Interrupt Priority Register"]
    pub iprior152: IPRIOR152,
    #[doc = "0x499 - Interrupt Priority Register"]
    pub iprior153: IPRIOR153,
    #[doc = "0x49a - Interrupt Priority Register"]
    pub iprior154: IPRIOR154,
    #[doc = "0x49b - Interrupt Priority Register"]
    pub iprior155: IPRIOR155,
    #[doc = "0x49c - Interrupt Priority Register"]
    pub iprior156: IPRIOR156,
    #[doc = "0x49d - Interrupt Priority Register"]
    pub iprior157: IPRIOR157,
    #[doc = "0x49e - Interrupt Priority Register"]
    pub iprior158: IPRIOR158,
    #[doc = "0x49f - Interrupt Priority Register"]
    pub iprior159: IPRIOR159,
    #[doc = "0x4a0 - Interrupt Priority Register"]
    pub iprior160: IPRIOR160,
    #[doc = "0x4a1 - Interrupt Priority Register"]
    pub iprior161: IPRIOR161,
    #[doc = "0x4a2 - Interrupt Priority Register"]
    pub iprior162: IPRIOR162,
    #[doc = "0x4a3 - Interrupt Priority Register"]
    pub iprior163: IPRIOR163,
    #[doc = "0x4a4 - Interrupt Priority Register"]
    pub iprior164: IPRIOR164,
    #[doc = "0x4a5 - Interrupt Priority Register"]
    pub iprior165: IPRIOR165,
    #[doc = "0x4a6 - Interrupt Priority Register"]
    pub iprior166: IPRIOR166,
    #[doc = "0x4a7 - Interrupt Priority Register"]
    pub iprior167: IPRIOR167,
    #[doc = "0x4a8 - Interrupt Priority Register"]
    pub iprior168: IPRIOR168,
    #[doc = "0x4a9 - Interrupt Priority Register"]
    pub iprior169: IPRIOR169,
    #[doc = "0x4aa - Interrupt Priority Register"]
    pub iprior170: IPRIOR170,
    #[doc = "0x4ab - Interrupt Priority Register"]
    pub iprior171: IPRIOR171,
    #[doc = "0x4ac - Interrupt Priority Register"]
    pub iprior172: IPRIOR172,
    #[doc = "0x4ad - Interrupt Priority Register"]
    pub iprior173: IPRIOR173,
    #[doc = "0x4ae - Interrupt Priority Register"]
    pub iprior174: IPRIOR174,
    #[doc = "0x4af - Interrupt Priority Register"]
    pub iprior175: IPRIOR175,
    #[doc = "0x4b0 - Interrupt Priority Register"]
    pub iprior176: IPRIOR176,
    #[doc = "0x4b1 - Interrupt Priority Register"]
    pub iprior177: IPRIOR177,
    #[doc = "0x4b2 - Interrupt Priority Register"]
    pub iprior178: IPRIOR178,
    #[doc = "0x4b3 - Interrupt Priority Register"]
    pub iprior179: IPRIOR179,
    #[doc = "0x4b4 - Interrupt Priority Register"]
    pub iprior180: IPRIOR180,
    #[doc = "0x4b5 - Interrupt Priority Register"]
    pub iprior181: IPRIOR181,
    #[doc = "0x4b6 - Interrupt Priority Register"]
    pub iprior182: IPRIOR182,
    #[doc = "0x4b7 - Interrupt Priority Register"]
    pub iprior183: IPRIOR183,
    #[doc = "0x4b8 - Interrupt Priority Register"]
    pub iprior184: IPRIOR184,
    #[doc = "0x4b9 - Interrupt Priority Register"]
    pub iprior185: IPRIOR185,
    #[doc = "0x4ba - Interrupt Priority Register"]
    pub iprior186: IPRIOR186,
    #[doc = "0x4bb - Interrupt Priority Register"]
    pub iprior187: IPRIOR187,
    #[doc = "0x4bc - Interrupt Priority Register"]
    pub iprior188: IPRIOR188,
    #[doc = "0x4bd - Interrupt Priority Register"]
    pub iprior189: IPRIOR189,
    #[doc = "0x4be - Interrupt Priority Register"]
    pub iprior190: IPRIOR190,
    #[doc = "0x4bf - Interrupt Priority Register"]
    pub iprior191: IPRIOR191,
    #[doc = "0x4c0 - Interrupt Priority Register"]
    pub iprior192: IPRIOR192,
    #[doc = "0x4c1 - Interrupt Priority Register"]
    pub iprior193: IPRIOR193,
    #[doc = "0x4c2 - Interrupt Priority Register"]
    pub iprior194: IPRIOR194,
    #[doc = "0x4c3 - Interrupt Priority Register"]
    pub iprior195: IPRIOR195,
    #[doc = "0x4c4 - Interrupt Priority Register"]
    pub iprior196: IPRIOR196,
    #[doc = "0x4c5 - Interrupt Priority Register"]
    pub iprior197: IPRIOR197,
    #[doc = "0x4c6 - Interrupt Priority Register"]
    pub iprior198: IPRIOR198,
    #[doc = "0x4c7 - Interrupt Priority Register"]
    pub iprior199: IPRIOR199,
    #[doc = "0x4c8 - Interrupt Priority Register"]
    pub iprior200: IPRIOR200,
    #[doc = "0x4c9 - Interrupt Priority Register"]
    pub iprior201: IPRIOR201,
    #[doc = "0x4ca - Interrupt Priority Register"]
    pub iprior202: IPRIOR202,
    #[doc = "0x4cb - Interrupt Priority Register"]
    pub iprior203: IPRIOR203,
    #[doc = "0x4cc - Interrupt Priority Register"]
    pub iprior204: IPRIOR204,
    #[doc = "0x4cd - Interrupt Priority Register"]
    pub iprior205: IPRIOR205,
    #[doc = "0x4ce - Interrupt Priority Register"]
    pub iprior206: IPRIOR206,
    #[doc = "0x4cf - Interrupt Priority Register"]
    pub iprior207: IPRIOR207,
    #[doc = "0x4d0 - Interrupt Priority Register"]
    pub iprior208: IPRIOR208,
    #[doc = "0x4d1 - Interrupt Priority Register"]
    pub iprior209: IPRIOR209,
    #[doc = "0x4d2 - Interrupt Priority Register"]
    pub iprior210: IPRIOR210,
    #[doc = "0x4d3 - Interrupt Priority Register"]
    pub iprior211: IPRIOR211,
    #[doc = "0x4d4 - Interrupt Priority Register"]
    pub iprior212: IPRIOR212,
    #[doc = "0x4d5 - Interrupt Priority Register"]
    pub iprior213: IPRIOR213,
    #[doc = "0x4d6 - Interrupt Priority Register"]
    pub iprior214: IPRIOR214,
    #[doc = "0x4d7 - Interrupt Priority Register"]
    pub iprior215: IPRIOR215,
    #[doc = "0x4d8 - Interrupt Priority Register"]
    pub iprior216: IPRIOR216,
    #[doc = "0x4d9 - Interrupt Priority Register"]
    pub iprior217: IPRIOR217,
    #[doc = "0x4da - Interrupt Priority Register"]
    pub iprior218: IPRIOR218,
    #[doc = "0x4db - Interrupt Priority Register"]
    pub iprior219: IPRIOR219,
    #[doc = "0x4dc - Interrupt Priority Register"]
    pub iprior220: IPRIOR220,
    #[doc = "0x4dd - Interrupt Priority Register"]
    pub iprior221: IPRIOR221,
    #[doc = "0x4de - Interrupt Priority Register"]
    pub iprior222: IPRIOR222,
    #[doc = "0x4df - Interrupt Priority Register"]
    pub iprior223: IPRIOR223,
    #[doc = "0x4e0 - Interrupt Priority Register"]
    pub iprior224: IPRIOR224,
    #[doc = "0x4e1 - Interrupt Priority Register"]
    pub iprior225: IPRIOR225,
    #[doc = "0x4e2 - Interrupt Priority Register"]
    pub iprior226: IPRIOR226,
    #[doc = "0x4e3 - Interrupt Priority Register"]
    pub iprior227: IPRIOR227,
    #[doc = "0x4e4 - Interrupt Priority Register"]
    pub iprior228: IPRIOR228,
    #[doc = "0x4e5 - Interrupt Priority Register"]
    pub iprior229: IPRIOR229,
    #[doc = "0x4e6 - Interrupt Priority Register"]
    pub iprior230: IPRIOR230,
    #[doc = "0x4e7 - Interrupt Priority Register"]
    pub iprior231: IPRIOR231,
    #[doc = "0x4e8 - Interrupt Priority Register"]
    pub iprior232: IPRIOR232,
    #[doc = "0x4e9 - Interrupt Priority Register"]
    pub iprior233: IPRIOR233,
    #[doc = "0x4ea - Interrupt Priority Register"]
    pub iprior234: IPRIOR234,
    #[doc = "0x4eb - Interrupt Priority Register"]
    pub iprior235: IPRIOR235,
    #[doc = "0x4ec - Interrupt Priority Register"]
    pub iprior236: IPRIOR236,
    #[doc = "0x4ed - Interrupt Priority Register"]
    pub iprior237: IPRIOR237,
    #[doc = "0x4ee - Interrupt Priority Register"]
    pub iprior238: IPRIOR238,
    #[doc = "0x4ef - Interrupt Priority Register"]
    pub iprior239: IPRIOR239,
    #[doc = "0x4f0 - Interrupt Priority Register"]
    pub iprior240: IPRIOR240,
    #[doc = "0x4f1 - Interrupt Priority Register"]
    pub iprior241: IPRIOR241,
    #[doc = "0x4f2 - Interrupt Priority Register"]
    pub iprior242: IPRIOR242,
    #[doc = "0x4f3 - Interrupt Priority Register"]
    pub iprior243: IPRIOR243,
    #[doc = "0x4f4 - Interrupt Priority Register"]
    pub iprior244: IPRIOR244,
    #[doc = "0x4f5 - Interrupt Priority Register"]
    pub iprior245: IPRIOR245,
    #[doc = "0x4f6 - Interrupt Priority Register"]
    pub iprior246: IPRIOR246,
    #[doc = "0x4f7 - Interrupt Priority Register"]
    pub iprior247: IPRIOR247,
    #[doc = "0x4f8 - Interrupt Priority Register"]
    pub iprior248: IPRIOR248,
    #[doc = "0x4f9 - Interrupt Priority Register"]
    pub iprior249: IPRIOR249,
    #[doc = "0x4fa - Interrupt Priority Register"]
    pub iprior250: IPRIOR250,
    #[doc = "0x4fb - Interrupt Priority Register"]
    pub iprior251: IPRIOR251,
    #[doc = "0x4fc - Interrupt Priority Register"]
    pub iprior252: IPRIOR252,
    #[doc = "0x4fd - Interrupt Priority Register"]
    pub iprior253: IPRIOR253,
    #[doc = "0x4fe - Interrupt Priority Register"]
    pub iprior254: IPRIOR254,
    #[doc = "0x4ff - Interrupt Priority Register"]
    pub iprior255: IPRIOR255,
    _reserved292: [u8; 0x0810],
    #[doc = "0xd10 - System Control Register"]
    pub sctlr: SCTLR,
    _reserved293: [u8; 0x02ec],
    #[doc = "0x1000 - System counter control register"]
    pub stk_ctlr: STK_CTLR,
    #[doc = "0x1004 - System START"]
    pub stk_sr: STK_SR,
    #[doc = "0x1008 - System counter low register"]
    pub stk_cntl: STK_CNTL,
    #[doc = "0x100c - System counter high register"]
    pub stk_cnth: STK_CNTH,
    #[doc = "0x1010 - System compare low register"]
    pub stk_cmplr: STK_CMPLR,
    #[doc = "0x1014 - System compare high register"]
    pub stk_cmphr: STK_CMPHR,
}
#[doc = "ISR1 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`]
module"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr1;
#[doc = "ISR2 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr2`]
module"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr2;
#[doc = "ISR3 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr3`]
module"]
pub type ISR3 = crate::Reg<isr3::ISR3_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr3;
#[doc = "ISR4 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr4`]
module"]
pub type ISR4 = crate::Reg<isr4::ISR4_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr4;
#[doc = "IPR1 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr1`]
module"]
pub type IPR1 = crate::Reg<ipr1::IPR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr1;
#[doc = "IPR2 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr2`]
module"]
pub type IPR2 = crate::Reg<ipr2::IPR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr2;
#[doc = "IPR3 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr3`]
module"]
pub type IPR3 = crate::Reg<ipr3::IPR3_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr3;
#[doc = "IPR4 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr4`]
module"]
pub type IPR4 = crate::Reg<ipr4::IPR4_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr4;
#[doc = "ITHRESDR (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ithresdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ithresdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ithresdr`]
module"]
pub type ITHRESDR = crate::Reg<ithresdr::ITHRESDR_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod ithresdr;
#[doc = "CFGR (w) register accessor: Interrupt Config Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Interrupt Config Register"]
pub mod cfgr;
#[doc = "GISR (r) register accessor: Interrupt Global Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gisr`]
module"]
pub type GISR = crate::Reg<gisr::GISR_SPEC>;
#[doc = "Interrupt Global Register"]
pub mod gisr;
#[doc = "VTFIDR (rw) register accessor: ID Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtfidr`]
module"]
pub type VTFIDR = crate::Reg<vtfidr::VTFIDR_SPEC>;
#[doc = "ID Config Register"]
pub mod vtfidr;
#[doc = "VTFADDRR0 (rw) register accessor: Interrupt 0 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtfaddrr0`]
module"]
pub type VTFADDRR0 = crate::Reg<vtfaddrr0::VTFADDRR0_SPEC>;
#[doc = "Interrupt 0 address Register"]
pub mod vtfaddrr0;
#[doc = "VTFADDRR1 (rw) register accessor: Interrupt 1 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtfaddrr1`]
module"]
pub type VTFADDRR1 = crate::Reg<vtfaddrr1::VTFADDRR1_SPEC>;
#[doc = "Interrupt 1 address Register"]
pub mod vtfaddrr1;
#[doc = "VTFADDRR2 (rw) register accessor: Interrupt 2 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtfaddrr2`]
module"]
pub type VTFADDRR2 = crate::Reg<vtfaddrr2::VTFADDRR2_SPEC>;
#[doc = "Interrupt 2 address Register"]
pub mod vtfaddrr2;
#[doc = "VTFADDRR3 (rw) register accessor: Interrupt 3 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtfaddrr3`]
module"]
pub type VTFADDRR3 = crate::Reg<vtfaddrr3::VTFADDRR3_SPEC>;
#[doc = "Interrupt 3 address Register"]
pub mod vtfaddrr3;
#[doc = "IENR1 (w) register accessor: Interrupt Setting Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr1`]
module"]
pub type IENR1 = crate::Reg<ienr1::IENR1_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr1;
#[doc = "IENR2 (w) register accessor: Interrupt Setting Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr2`]
module"]
pub type IENR2 = crate::Reg<ienr2::IENR2_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr2;
#[doc = "IENR3 (w) register accessor: Interrupt Setting Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr3`]
module"]
pub type IENR3 = crate::Reg<ienr3::IENR3_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr3;
#[doc = "IENR4 (w) register accessor: Interrupt Setting Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr4`]
module"]
pub type IENR4 = crate::Reg<ienr4::IENR4_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr4;
#[doc = "IRER1 (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irer1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer1`]
module"]
pub type IRER1 = crate::Reg<irer1::IRER1_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer1;
#[doc = "IRER2 (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irer2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer2`]
module"]
pub type IRER2 = crate::Reg<irer2::IRER2_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer2;
#[doc = "IRER3 (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irer3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer3`]
module"]
pub type IRER3 = crate::Reg<irer3::IRER3_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer3;
#[doc = "IRER4 (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irer4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer4`]
module"]
pub type IRER4 = crate::Reg<irer4::IRER4_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer4;
#[doc = "IPSR1 (w) register accessor: Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr1`]
module"]
pub type IPSR1 = crate::Reg<ipsr1::IPSR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr1;
#[doc = "IPSR2 (w) register accessor: Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr2`]
module"]
pub type IPSR2 = crate::Reg<ipsr2::IPSR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr2;
#[doc = "IPSR3 (w) register accessor: Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr3`]
module"]
pub type IPSR3 = crate::Reg<ipsr3::IPSR3_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr3;
#[doc = "IPSR4 (w) register accessor: Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr4`]
module"]
pub type IPSR4 = crate::Reg<ipsr4::IPSR4_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr4;
#[doc = "IPRR1 (w) register accessor: Interrupt Pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr1`]
module"]
pub type IPRR1 = crate::Reg<iprr1::IPRR1_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr1;
#[doc = "IPRR2 (w) register accessor: Interrupt Pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr2`]
module"]
pub type IPRR2 = crate::Reg<iprr2::IPRR2_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr2;
#[doc = "IPRR3 (w) register accessor: Interrupt Pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr3`]
module"]
pub type IPRR3 = crate::Reg<iprr3::IPRR3_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr3;
#[doc = "IPRR4 (w) register accessor: Interrupt Pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr4`]
module"]
pub type IPRR4 = crate::Reg<iprr4::IPRR4_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr4;
#[doc = "IACTR1 (w) register accessor: Interrupt ACTIVE Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iactr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr1`]
module"]
pub type IACTR1 = crate::Reg<iactr1::IACTR1_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr1;
#[doc = "IACTR2 (w) register accessor: Interrupt ACTIVE Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iactr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr2`]
module"]
pub type IACTR2 = crate::Reg<iactr2::IACTR2_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr2;
#[doc = "IACTR3 (w) register accessor: Interrupt ACTIVE Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iactr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr3`]
module"]
pub type IACTR3 = crate::Reg<iactr3::IACTR3_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr3;
#[doc = "IACTR4 (w) register accessor: Interrupt ACTIVE Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iactr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr4`]
module"]
pub type IACTR4 = crate::Reg<iactr4::IACTR4_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr4;
#[doc = "IPRIOR0 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior0`]
module"]
pub type IPRIOR0 = crate::Reg<iprior0::IPRIOR0_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior0;
#[doc = "IPRIOR1 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior1`]
module"]
pub type IPRIOR1 = crate::Reg<iprior1::IPRIOR1_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior1;
#[doc = "IPRIOR2 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior2`]
module"]
pub type IPRIOR2 = crate::Reg<iprior2::IPRIOR2_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior2;
#[doc = "IPRIOR3 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior3`]
module"]
pub type IPRIOR3 = crate::Reg<iprior3::IPRIOR3_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior3;
#[doc = "IPRIOR4 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior4`]
module"]
pub type IPRIOR4 = crate::Reg<iprior4::IPRIOR4_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior4;
#[doc = "IPRIOR5 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior5`]
module"]
pub type IPRIOR5 = crate::Reg<iprior5::IPRIOR5_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior5;
#[doc = "IPRIOR6 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior6`]
module"]
pub type IPRIOR6 = crate::Reg<iprior6::IPRIOR6_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior6;
#[doc = "IPRIOR7 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior7`]
module"]
pub type IPRIOR7 = crate::Reg<iprior7::IPRIOR7_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior7;
#[doc = "IPRIOR8 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior8`]
module"]
pub type IPRIOR8 = crate::Reg<iprior8::IPRIOR8_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior8;
#[doc = "IPRIOR9 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior9`]
module"]
pub type IPRIOR9 = crate::Reg<iprior9::IPRIOR9_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior9;
#[doc = "IPRIOR10 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior10`]
module"]
pub type IPRIOR10 = crate::Reg<iprior10::IPRIOR10_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior10;
#[doc = "IPRIOR11 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior11`]
module"]
pub type IPRIOR11 = crate::Reg<iprior11::IPRIOR11_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior11;
#[doc = "IPRIOR12 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior12`]
module"]
pub type IPRIOR12 = crate::Reg<iprior12::IPRIOR12_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior12;
#[doc = "IPRIOR13 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior13`]
module"]
pub type IPRIOR13 = crate::Reg<iprior13::IPRIOR13_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior13;
#[doc = "IPRIOR14 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior14`]
module"]
pub type IPRIOR14 = crate::Reg<iprior14::IPRIOR14_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior14;
#[doc = "IPRIOR15 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior15`]
module"]
pub type IPRIOR15 = crate::Reg<iprior15::IPRIOR15_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior15;
#[doc = "IPRIOR16 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior16`]
module"]
pub type IPRIOR16 = crate::Reg<iprior16::IPRIOR16_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior16;
#[doc = "IPRIOR17 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior17`]
module"]
pub type IPRIOR17 = crate::Reg<iprior17::IPRIOR17_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior17;
#[doc = "IPRIOR18 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior18`]
module"]
pub type IPRIOR18 = crate::Reg<iprior18::IPRIOR18_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior18;
#[doc = "IPRIOR19 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior19`]
module"]
pub type IPRIOR19 = crate::Reg<iprior19::IPRIOR19_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior19;
#[doc = "IPRIOR20 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior20`]
module"]
pub type IPRIOR20 = crate::Reg<iprior20::IPRIOR20_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior20;
#[doc = "IPRIOR21 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior21`]
module"]
pub type IPRIOR21 = crate::Reg<iprior21::IPRIOR21_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior21;
#[doc = "IPRIOR22 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior22`]
module"]
pub type IPRIOR22 = crate::Reg<iprior22::IPRIOR22_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior22;
#[doc = "IPRIOR23 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior23`]
module"]
pub type IPRIOR23 = crate::Reg<iprior23::IPRIOR23_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior23;
#[doc = "IPRIOR24 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior24`]
module"]
pub type IPRIOR24 = crate::Reg<iprior24::IPRIOR24_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior24;
#[doc = "IPRIOR25 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior25`]
module"]
pub type IPRIOR25 = crate::Reg<iprior25::IPRIOR25_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior25;
#[doc = "IPRIOR26 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior26`]
module"]
pub type IPRIOR26 = crate::Reg<iprior26::IPRIOR26_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior26;
#[doc = "IPRIOR27 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior27`]
module"]
pub type IPRIOR27 = crate::Reg<iprior27::IPRIOR27_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior27;
#[doc = "IPRIOR28 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior28`]
module"]
pub type IPRIOR28 = crate::Reg<iprior28::IPRIOR28_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior28;
#[doc = "IPRIOR29 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior29`]
module"]
pub type IPRIOR29 = crate::Reg<iprior29::IPRIOR29_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior29;
#[doc = "IPRIOR30 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior30`]
module"]
pub type IPRIOR30 = crate::Reg<iprior30::IPRIOR30_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior30;
#[doc = "IPRIOR31 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior31`]
module"]
pub type IPRIOR31 = crate::Reg<iprior31::IPRIOR31_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior31;
#[doc = "IPRIOR32 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior32`]
module"]
pub type IPRIOR32 = crate::Reg<iprior32::IPRIOR32_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior32;
#[doc = "IPRIOR33 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior33`]
module"]
pub type IPRIOR33 = crate::Reg<iprior33::IPRIOR33_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior33;
#[doc = "IPRIOR34 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior34`]
module"]
pub type IPRIOR34 = crate::Reg<iprior34::IPRIOR34_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior34;
#[doc = "IPRIOR35 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior35`]
module"]
pub type IPRIOR35 = crate::Reg<iprior35::IPRIOR35_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior35;
#[doc = "IPRIOR36 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior36`]
module"]
pub type IPRIOR36 = crate::Reg<iprior36::IPRIOR36_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior36;
#[doc = "IPRIOR37 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior37`]
module"]
pub type IPRIOR37 = crate::Reg<iprior37::IPRIOR37_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior37;
#[doc = "IPRIOR38 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior38`]
module"]
pub type IPRIOR38 = crate::Reg<iprior38::IPRIOR38_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior38;
#[doc = "IPRIOR39 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior39`]
module"]
pub type IPRIOR39 = crate::Reg<iprior39::IPRIOR39_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior39;
#[doc = "IPRIOR40 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior40`]
module"]
pub type IPRIOR40 = crate::Reg<iprior40::IPRIOR40_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior40;
#[doc = "IPRIOR41 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior41`]
module"]
pub type IPRIOR41 = crate::Reg<iprior41::IPRIOR41_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior41;
#[doc = "IPRIOR42 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior42`]
module"]
pub type IPRIOR42 = crate::Reg<iprior42::IPRIOR42_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior42;
#[doc = "IPRIOR43 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior43`]
module"]
pub type IPRIOR43 = crate::Reg<iprior43::IPRIOR43_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior43;
#[doc = "IPRIOR44 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior44`]
module"]
pub type IPRIOR44 = crate::Reg<iprior44::IPRIOR44_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior44;
#[doc = "IPRIOR45 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior45`]
module"]
pub type IPRIOR45 = crate::Reg<iprior45::IPRIOR45_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior45;
#[doc = "IPRIOR46 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior46`]
module"]
pub type IPRIOR46 = crate::Reg<iprior46::IPRIOR46_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior46;
#[doc = "IPRIOR47 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior47`]
module"]
pub type IPRIOR47 = crate::Reg<iprior47::IPRIOR47_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior47;
#[doc = "IPRIOR48 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior48`]
module"]
pub type IPRIOR48 = crate::Reg<iprior48::IPRIOR48_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior48;
#[doc = "IPRIOR49 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior49`]
module"]
pub type IPRIOR49 = crate::Reg<iprior49::IPRIOR49_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior49;
#[doc = "IPRIOR50 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior50`]
module"]
pub type IPRIOR50 = crate::Reg<iprior50::IPRIOR50_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior50;
#[doc = "IPRIOR51 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior51`]
module"]
pub type IPRIOR51 = crate::Reg<iprior51::IPRIOR51_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior51;
#[doc = "IPRIOR52 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior52`]
module"]
pub type IPRIOR52 = crate::Reg<iprior52::IPRIOR52_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior52;
#[doc = "IPRIOR53 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior53`]
module"]
pub type IPRIOR53 = crate::Reg<iprior53::IPRIOR53_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior53;
#[doc = "IPRIOR54 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior54`]
module"]
pub type IPRIOR54 = crate::Reg<iprior54::IPRIOR54_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior54;
#[doc = "IPRIOR55 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior55`]
module"]
pub type IPRIOR55 = crate::Reg<iprior55::IPRIOR55_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior55;
#[doc = "IPRIOR56 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior56`]
module"]
pub type IPRIOR56 = crate::Reg<iprior56::IPRIOR56_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior56;
#[doc = "IPRIOR57 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior57`]
module"]
pub type IPRIOR57 = crate::Reg<iprior57::IPRIOR57_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior57;
#[doc = "IPRIOR58 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior58`]
module"]
pub type IPRIOR58 = crate::Reg<iprior58::IPRIOR58_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior58;
#[doc = "IPRIOR59 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior59`]
module"]
pub type IPRIOR59 = crate::Reg<iprior59::IPRIOR59_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior59;
#[doc = "IPRIOR60 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior60`]
module"]
pub type IPRIOR60 = crate::Reg<iprior60::IPRIOR60_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior60;
#[doc = "IPRIOR61 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior61`]
module"]
pub type IPRIOR61 = crate::Reg<iprior61::IPRIOR61_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior61;
#[doc = "IPRIOR62 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior62`]
module"]
pub type IPRIOR62 = crate::Reg<iprior62::IPRIOR62_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior62;
#[doc = "IPRIOR63 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior63`]
module"]
pub type IPRIOR63 = crate::Reg<iprior63::IPRIOR63_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior63;
#[doc = "IPRIOR64 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior64`]
module"]
pub type IPRIOR64 = crate::Reg<iprior64::IPRIOR64_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior64;
#[doc = "IPRIOR65 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior65`]
module"]
pub type IPRIOR65 = crate::Reg<iprior65::IPRIOR65_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior65;
#[doc = "IPRIOR66 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior66`]
module"]
pub type IPRIOR66 = crate::Reg<iprior66::IPRIOR66_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior66;
#[doc = "IPRIOR67 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior67`]
module"]
pub type IPRIOR67 = crate::Reg<iprior67::IPRIOR67_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior67;
#[doc = "IPRIOR68 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior68`]
module"]
pub type IPRIOR68 = crate::Reg<iprior68::IPRIOR68_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior68;
#[doc = "IPRIOR69 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior69`]
module"]
pub type IPRIOR69 = crate::Reg<iprior69::IPRIOR69_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior69;
#[doc = "IPRIOR70 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior70`]
module"]
pub type IPRIOR70 = crate::Reg<iprior70::IPRIOR70_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior70;
#[doc = "IPRIOR71 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior71`]
module"]
pub type IPRIOR71 = crate::Reg<iprior71::IPRIOR71_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior71;
#[doc = "IPRIOR72 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior72`]
module"]
pub type IPRIOR72 = crate::Reg<iprior72::IPRIOR72_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior72;
#[doc = "IPRIOR73 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior73::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior73`]
module"]
pub type IPRIOR73 = crate::Reg<iprior73::IPRIOR73_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior73;
#[doc = "IPRIOR74 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior74`]
module"]
pub type IPRIOR74 = crate::Reg<iprior74::IPRIOR74_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior74;
#[doc = "IPRIOR75 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior75`]
module"]
pub type IPRIOR75 = crate::Reg<iprior75::IPRIOR75_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior75;
#[doc = "IPRIOR76 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior76`]
module"]
pub type IPRIOR76 = crate::Reg<iprior76::IPRIOR76_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior76;
#[doc = "IPRIOR77 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior77`]
module"]
pub type IPRIOR77 = crate::Reg<iprior77::IPRIOR77_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior77;
#[doc = "IPRIOR78 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior78`]
module"]
pub type IPRIOR78 = crate::Reg<iprior78::IPRIOR78_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior78;
#[doc = "IPRIOR79 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior79`]
module"]
pub type IPRIOR79 = crate::Reg<iprior79::IPRIOR79_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior79;
#[doc = "IPRIOR80 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior80`]
module"]
pub type IPRIOR80 = crate::Reg<iprior80::IPRIOR80_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior80;
#[doc = "IPRIOR81 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior81`]
module"]
pub type IPRIOR81 = crate::Reg<iprior81::IPRIOR81_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior81;
#[doc = "IPRIOR82 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior82`]
module"]
pub type IPRIOR82 = crate::Reg<iprior82::IPRIOR82_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior82;
#[doc = "IPRIOR83 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior83`]
module"]
pub type IPRIOR83 = crate::Reg<iprior83::IPRIOR83_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior83;
#[doc = "IPRIOR84 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior84`]
module"]
pub type IPRIOR84 = crate::Reg<iprior84::IPRIOR84_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior84;
#[doc = "IPRIOR85 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior85`]
module"]
pub type IPRIOR85 = crate::Reg<iprior85::IPRIOR85_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior85;
#[doc = "IPRIOR86 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior86`]
module"]
pub type IPRIOR86 = crate::Reg<iprior86::IPRIOR86_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior86;
#[doc = "IPRIOR87 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior87`]
module"]
pub type IPRIOR87 = crate::Reg<iprior87::IPRIOR87_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior87;
#[doc = "IPRIOR88 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior88`]
module"]
pub type IPRIOR88 = crate::Reg<iprior88::IPRIOR88_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior88;
#[doc = "IPRIOR89 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior89`]
module"]
pub type IPRIOR89 = crate::Reg<iprior89::IPRIOR89_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior89;
#[doc = "IPRIOR90 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior90`]
module"]
pub type IPRIOR90 = crate::Reg<iprior90::IPRIOR90_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior90;
#[doc = "IPRIOR91 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior91`]
module"]
pub type IPRIOR91 = crate::Reg<iprior91::IPRIOR91_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior91;
#[doc = "IPRIOR92 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior92`]
module"]
pub type IPRIOR92 = crate::Reg<iprior92::IPRIOR92_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior92;
#[doc = "IPRIOR93 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior93`]
module"]
pub type IPRIOR93 = crate::Reg<iprior93::IPRIOR93_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior93;
#[doc = "IPRIOR94 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior94`]
module"]
pub type IPRIOR94 = crate::Reg<iprior94::IPRIOR94_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior94;
#[doc = "IPRIOR95 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior95`]
module"]
pub type IPRIOR95 = crate::Reg<iprior95::IPRIOR95_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior95;
#[doc = "IPRIOR96 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior96`]
module"]
pub type IPRIOR96 = crate::Reg<iprior96::IPRIOR96_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior96;
#[doc = "IPRIOR97 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior97`]
module"]
pub type IPRIOR97 = crate::Reg<iprior97::IPRIOR97_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior97;
#[doc = "IPRIOR98 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior98`]
module"]
pub type IPRIOR98 = crate::Reg<iprior98::IPRIOR98_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior98;
#[doc = "IPRIOR99 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior99`]
module"]
pub type IPRIOR99 = crate::Reg<iprior99::IPRIOR99_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior99;
#[doc = "IPRIOR100 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior100`]
module"]
pub type IPRIOR100 = crate::Reg<iprior100::IPRIOR100_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior100;
#[doc = "IPRIOR101 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior101`]
module"]
pub type IPRIOR101 = crate::Reg<iprior101::IPRIOR101_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior101;
#[doc = "IPRIOR102 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior102`]
module"]
pub type IPRIOR102 = crate::Reg<iprior102::IPRIOR102_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior102;
#[doc = "IPRIOR103 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior103`]
module"]
pub type IPRIOR103 = crate::Reg<iprior103::IPRIOR103_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior103;
#[doc = "IPRIOR104 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior104`]
module"]
pub type IPRIOR104 = crate::Reg<iprior104::IPRIOR104_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior104;
#[doc = "IPRIOR105 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior105`]
module"]
pub type IPRIOR105 = crate::Reg<iprior105::IPRIOR105_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior105;
#[doc = "IPRIOR106 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior106`]
module"]
pub type IPRIOR106 = crate::Reg<iprior106::IPRIOR106_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior106;
#[doc = "IPRIOR107 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior107`]
module"]
pub type IPRIOR107 = crate::Reg<iprior107::IPRIOR107_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior107;
#[doc = "IPRIOR108 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior108`]
module"]
pub type IPRIOR108 = crate::Reg<iprior108::IPRIOR108_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior108;
#[doc = "IPRIOR109 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior109`]
module"]
pub type IPRIOR109 = crate::Reg<iprior109::IPRIOR109_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior109;
#[doc = "IPRIOR110 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior110`]
module"]
pub type IPRIOR110 = crate::Reg<iprior110::IPRIOR110_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior110;
#[doc = "IPRIOR111 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior111`]
module"]
pub type IPRIOR111 = crate::Reg<iprior111::IPRIOR111_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior111;
#[doc = "IPRIOR112 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior112`]
module"]
pub type IPRIOR112 = crate::Reg<iprior112::IPRIOR112_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior112;
#[doc = "IPRIOR113 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior113`]
module"]
pub type IPRIOR113 = crate::Reg<iprior113::IPRIOR113_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior113;
#[doc = "IPRIOR114 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior114`]
module"]
pub type IPRIOR114 = crate::Reg<iprior114::IPRIOR114_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior114;
#[doc = "IPRIOR115 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior115`]
module"]
pub type IPRIOR115 = crate::Reg<iprior115::IPRIOR115_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior115;
#[doc = "IPRIOR116 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior116::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior116`]
module"]
pub type IPRIOR116 = crate::Reg<iprior116::IPRIOR116_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior116;
#[doc = "IPRIOR117 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior117::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior117`]
module"]
pub type IPRIOR117 = crate::Reg<iprior117::IPRIOR117_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior117;
#[doc = "IPRIOR118 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior118::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior118`]
module"]
pub type IPRIOR118 = crate::Reg<iprior118::IPRIOR118_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior118;
#[doc = "IPRIOR119 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior119::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior119`]
module"]
pub type IPRIOR119 = crate::Reg<iprior119::IPRIOR119_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior119;
#[doc = "IPRIOR120 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior120`]
module"]
pub type IPRIOR120 = crate::Reg<iprior120::IPRIOR120_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior120;
#[doc = "IPRIOR121 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior121`]
module"]
pub type IPRIOR121 = crate::Reg<iprior121::IPRIOR121_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior121;
#[doc = "IPRIOR122 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior122`]
module"]
pub type IPRIOR122 = crate::Reg<iprior122::IPRIOR122_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior122;
#[doc = "IPRIOR123 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior123`]
module"]
pub type IPRIOR123 = crate::Reg<iprior123::IPRIOR123_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior123;
#[doc = "IPRIOR124 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior124`]
module"]
pub type IPRIOR124 = crate::Reg<iprior124::IPRIOR124_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior124;
#[doc = "IPRIOR125 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior125`]
module"]
pub type IPRIOR125 = crate::Reg<iprior125::IPRIOR125_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior125;
#[doc = "IPRIOR126 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior126`]
module"]
pub type IPRIOR126 = crate::Reg<iprior126::IPRIOR126_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior126;
#[doc = "IPRIOR127 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior127`]
module"]
pub type IPRIOR127 = crate::Reg<iprior127::IPRIOR127_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior127;
#[doc = "IPRIOR128 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior128`]
module"]
pub type IPRIOR128 = crate::Reg<iprior128::IPRIOR128_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior128;
#[doc = "IPRIOR129 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior129`]
module"]
pub type IPRIOR129 = crate::Reg<iprior129::IPRIOR129_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior129;
#[doc = "IPRIOR130 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior130`]
module"]
pub type IPRIOR130 = crate::Reg<iprior130::IPRIOR130_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior130;
#[doc = "IPRIOR131 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior131`]
module"]
pub type IPRIOR131 = crate::Reg<iprior131::IPRIOR131_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior131;
#[doc = "IPRIOR132 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior132`]
module"]
pub type IPRIOR132 = crate::Reg<iprior132::IPRIOR132_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior132;
#[doc = "IPRIOR133 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior133`]
module"]
pub type IPRIOR133 = crate::Reg<iprior133::IPRIOR133_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior133;
#[doc = "IPRIOR134 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior134`]
module"]
pub type IPRIOR134 = crate::Reg<iprior134::IPRIOR134_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior134;
#[doc = "IPRIOR135 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior135`]
module"]
pub type IPRIOR135 = crate::Reg<iprior135::IPRIOR135_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior135;
#[doc = "IPRIOR136 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior136`]
module"]
pub type IPRIOR136 = crate::Reg<iprior136::IPRIOR136_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior136;
#[doc = "IPRIOR137 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior137`]
module"]
pub type IPRIOR137 = crate::Reg<iprior137::IPRIOR137_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior137;
#[doc = "IPRIOR138 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior138`]
module"]
pub type IPRIOR138 = crate::Reg<iprior138::IPRIOR138_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior138;
#[doc = "IPRIOR139 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior139`]
module"]
pub type IPRIOR139 = crate::Reg<iprior139::IPRIOR139_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior139;
#[doc = "IPRIOR140 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior140`]
module"]
pub type IPRIOR140 = crate::Reg<iprior140::IPRIOR140_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior140;
#[doc = "IPRIOR141 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior141::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior141`]
module"]
pub type IPRIOR141 = crate::Reg<iprior141::IPRIOR141_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior141;
#[doc = "IPRIOR142 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior142::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior142::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior142`]
module"]
pub type IPRIOR142 = crate::Reg<iprior142::IPRIOR142_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior142;
#[doc = "IPRIOR143 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior143::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior143`]
module"]
pub type IPRIOR143 = crate::Reg<iprior143::IPRIOR143_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior143;
#[doc = "IPRIOR144 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior144::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior144`]
module"]
pub type IPRIOR144 = crate::Reg<iprior144::IPRIOR144_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior144;
#[doc = "IPRIOR145 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior145::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior145`]
module"]
pub type IPRIOR145 = crate::Reg<iprior145::IPRIOR145_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior145;
#[doc = "IPRIOR146 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior146::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior146`]
module"]
pub type IPRIOR146 = crate::Reg<iprior146::IPRIOR146_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior146;
#[doc = "IPRIOR147 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior147::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior147`]
module"]
pub type IPRIOR147 = crate::Reg<iprior147::IPRIOR147_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior147;
#[doc = "IPRIOR148 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior148::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior148`]
module"]
pub type IPRIOR148 = crate::Reg<iprior148::IPRIOR148_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior148;
#[doc = "IPRIOR149 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior149::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior149`]
module"]
pub type IPRIOR149 = crate::Reg<iprior149::IPRIOR149_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior149;
#[doc = "IPRIOR150 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior150::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior150`]
module"]
pub type IPRIOR150 = crate::Reg<iprior150::IPRIOR150_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior150;
#[doc = "IPRIOR151 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior151::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior151`]
module"]
pub type IPRIOR151 = crate::Reg<iprior151::IPRIOR151_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior151;
#[doc = "IPRIOR152 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior152::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior152`]
module"]
pub type IPRIOR152 = crate::Reg<iprior152::IPRIOR152_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior152;
#[doc = "IPRIOR153 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior153::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior153`]
module"]
pub type IPRIOR153 = crate::Reg<iprior153::IPRIOR153_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior153;
#[doc = "IPRIOR154 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior154::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior154`]
module"]
pub type IPRIOR154 = crate::Reg<iprior154::IPRIOR154_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior154;
#[doc = "IPRIOR155 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior155`]
module"]
pub type IPRIOR155 = crate::Reg<iprior155::IPRIOR155_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior155;
#[doc = "IPRIOR156 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior156`]
module"]
pub type IPRIOR156 = crate::Reg<iprior156::IPRIOR156_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior156;
#[doc = "IPRIOR157 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior157`]
module"]
pub type IPRIOR157 = crate::Reg<iprior157::IPRIOR157_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior157;
#[doc = "IPRIOR158 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior158`]
module"]
pub type IPRIOR158 = crate::Reg<iprior158::IPRIOR158_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior158;
#[doc = "IPRIOR159 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior159`]
module"]
pub type IPRIOR159 = crate::Reg<iprior159::IPRIOR159_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior159;
#[doc = "IPRIOR160 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior160`]
module"]
pub type IPRIOR160 = crate::Reg<iprior160::IPRIOR160_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior160;
#[doc = "IPRIOR161 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior161::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior161`]
module"]
pub type IPRIOR161 = crate::Reg<iprior161::IPRIOR161_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior161;
#[doc = "IPRIOR162 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior162::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior162`]
module"]
pub type IPRIOR162 = crate::Reg<iprior162::IPRIOR162_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior162;
#[doc = "IPRIOR163 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior163::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior163`]
module"]
pub type IPRIOR163 = crate::Reg<iprior163::IPRIOR163_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior163;
#[doc = "IPRIOR164 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior164::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior164`]
module"]
pub type IPRIOR164 = crate::Reg<iprior164::IPRIOR164_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior164;
#[doc = "IPRIOR165 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior165::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior165`]
module"]
pub type IPRIOR165 = crate::Reg<iprior165::IPRIOR165_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior165;
#[doc = "IPRIOR166 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior166::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior166`]
module"]
pub type IPRIOR166 = crate::Reg<iprior166::IPRIOR166_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior166;
#[doc = "IPRIOR167 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior167::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior167`]
module"]
pub type IPRIOR167 = crate::Reg<iprior167::IPRIOR167_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior167;
#[doc = "IPRIOR168 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior168::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior168`]
module"]
pub type IPRIOR168 = crate::Reg<iprior168::IPRIOR168_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior168;
#[doc = "IPRIOR169 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior169::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior169`]
module"]
pub type IPRIOR169 = crate::Reg<iprior169::IPRIOR169_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior169;
#[doc = "IPRIOR170 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior170::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior170::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior170`]
module"]
pub type IPRIOR170 = crate::Reg<iprior170::IPRIOR170_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior170;
#[doc = "IPRIOR171 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior171::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior171::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior171`]
module"]
pub type IPRIOR171 = crate::Reg<iprior171::IPRIOR171_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior171;
#[doc = "IPRIOR172 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior172::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior172::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior172`]
module"]
pub type IPRIOR172 = crate::Reg<iprior172::IPRIOR172_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior172;
#[doc = "IPRIOR173 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior173::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior173::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior173`]
module"]
pub type IPRIOR173 = crate::Reg<iprior173::IPRIOR173_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior173;
#[doc = "IPRIOR174 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior174::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior174::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior174`]
module"]
pub type IPRIOR174 = crate::Reg<iprior174::IPRIOR174_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior174;
#[doc = "IPRIOR175 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior175::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior175::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior175`]
module"]
pub type IPRIOR175 = crate::Reg<iprior175::IPRIOR175_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior175;
#[doc = "IPRIOR176 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior176::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior176`]
module"]
pub type IPRIOR176 = crate::Reg<iprior176::IPRIOR176_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior176;
#[doc = "IPRIOR177 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior177::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior177::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior177`]
module"]
pub type IPRIOR177 = crate::Reg<iprior177::IPRIOR177_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior177;
#[doc = "IPRIOR178 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior178::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior178::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior178`]
module"]
pub type IPRIOR178 = crate::Reg<iprior178::IPRIOR178_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior178;
#[doc = "IPRIOR179 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior179::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior179::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior179`]
module"]
pub type IPRIOR179 = crate::Reg<iprior179::IPRIOR179_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior179;
#[doc = "IPRIOR180 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior180::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior180::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior180`]
module"]
pub type IPRIOR180 = crate::Reg<iprior180::IPRIOR180_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior180;
#[doc = "IPRIOR181 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior181::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior181::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior181`]
module"]
pub type IPRIOR181 = crate::Reg<iprior181::IPRIOR181_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior181;
#[doc = "IPRIOR182 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior182::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior182::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior182`]
module"]
pub type IPRIOR182 = crate::Reg<iprior182::IPRIOR182_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior182;
#[doc = "IPRIOR183 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior183::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior183::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior183`]
module"]
pub type IPRIOR183 = crate::Reg<iprior183::IPRIOR183_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior183;
#[doc = "IPRIOR184 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior184::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior184::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior184`]
module"]
pub type IPRIOR184 = crate::Reg<iprior184::IPRIOR184_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior184;
#[doc = "IPRIOR185 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior185::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior185::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior185`]
module"]
pub type IPRIOR185 = crate::Reg<iprior185::IPRIOR185_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior185;
#[doc = "IPRIOR186 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior186::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior186`]
module"]
pub type IPRIOR186 = crate::Reg<iprior186::IPRIOR186_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior186;
#[doc = "IPRIOR187 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior187::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior187`]
module"]
pub type IPRIOR187 = crate::Reg<iprior187::IPRIOR187_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior187;
#[doc = "IPRIOR188 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior188::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior188`]
module"]
pub type IPRIOR188 = crate::Reg<iprior188::IPRIOR188_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior188;
#[doc = "IPRIOR189 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior189::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior189`]
module"]
pub type IPRIOR189 = crate::Reg<iprior189::IPRIOR189_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior189;
#[doc = "IPRIOR190 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior190::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior190`]
module"]
pub type IPRIOR190 = crate::Reg<iprior190::IPRIOR190_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior190;
#[doc = "IPRIOR191 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior191::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior191`]
module"]
pub type IPRIOR191 = crate::Reg<iprior191::IPRIOR191_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior191;
#[doc = "IPRIOR192 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior192::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior192`]
module"]
pub type IPRIOR192 = crate::Reg<iprior192::IPRIOR192_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior192;
#[doc = "IPRIOR193 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior193::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior193::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior193`]
module"]
pub type IPRIOR193 = crate::Reg<iprior193::IPRIOR193_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior193;
#[doc = "IPRIOR194 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior194::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior194::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior194`]
module"]
pub type IPRIOR194 = crate::Reg<iprior194::IPRIOR194_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior194;
#[doc = "IPRIOR195 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior195::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior195::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior195`]
module"]
pub type IPRIOR195 = crate::Reg<iprior195::IPRIOR195_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior195;
#[doc = "IPRIOR196 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior196::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior196::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior196`]
module"]
pub type IPRIOR196 = crate::Reg<iprior196::IPRIOR196_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior196;
#[doc = "IPRIOR197 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior197::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior197::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior197`]
module"]
pub type IPRIOR197 = crate::Reg<iprior197::IPRIOR197_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior197;
#[doc = "IPRIOR198 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior198::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior198::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior198`]
module"]
pub type IPRIOR198 = crate::Reg<iprior198::IPRIOR198_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior198;
#[doc = "IPRIOR199 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior199::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior199`]
module"]
pub type IPRIOR199 = crate::Reg<iprior199::IPRIOR199_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior199;
#[doc = "IPRIOR200 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior200::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior200::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior200`]
module"]
pub type IPRIOR200 = crate::Reg<iprior200::IPRIOR200_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior200;
#[doc = "IPRIOR201 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior201::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior201::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior201`]
module"]
pub type IPRIOR201 = crate::Reg<iprior201::IPRIOR201_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior201;
#[doc = "IPRIOR202 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior202::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior202::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior202`]
module"]
pub type IPRIOR202 = crate::Reg<iprior202::IPRIOR202_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior202;
#[doc = "IPRIOR203 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior203::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior203::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior203`]
module"]
pub type IPRIOR203 = crate::Reg<iprior203::IPRIOR203_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior203;
#[doc = "IPRIOR204 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior204::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior204::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior204`]
module"]
pub type IPRIOR204 = crate::Reg<iprior204::IPRIOR204_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior204;
#[doc = "IPRIOR205 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior205::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior205::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior205`]
module"]
pub type IPRIOR205 = crate::Reg<iprior205::IPRIOR205_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior205;
#[doc = "IPRIOR206 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior206::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior206::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior206`]
module"]
pub type IPRIOR206 = crate::Reg<iprior206::IPRIOR206_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior206;
#[doc = "IPRIOR207 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior207::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior207::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior207`]
module"]
pub type IPRIOR207 = crate::Reg<iprior207::IPRIOR207_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior207;
#[doc = "IPRIOR208 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior208::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior208::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior208`]
module"]
pub type IPRIOR208 = crate::Reg<iprior208::IPRIOR208_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior208;
#[doc = "IPRIOR209 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior209::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior209::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior209`]
module"]
pub type IPRIOR209 = crate::Reg<iprior209::IPRIOR209_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior209;
#[doc = "IPRIOR210 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior210::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior210::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior210`]
module"]
pub type IPRIOR210 = crate::Reg<iprior210::IPRIOR210_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior210;
#[doc = "IPRIOR211 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior211::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior211::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior211`]
module"]
pub type IPRIOR211 = crate::Reg<iprior211::IPRIOR211_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior211;
#[doc = "IPRIOR212 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior212::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior212::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior212`]
module"]
pub type IPRIOR212 = crate::Reg<iprior212::IPRIOR212_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior212;
#[doc = "IPRIOR213 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior213::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior213::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior213`]
module"]
pub type IPRIOR213 = crate::Reg<iprior213::IPRIOR213_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior213;
#[doc = "IPRIOR214 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior214::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior214::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior214`]
module"]
pub type IPRIOR214 = crate::Reg<iprior214::IPRIOR214_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior214;
#[doc = "IPRIOR215 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior215::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior215::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior215`]
module"]
pub type IPRIOR215 = crate::Reg<iprior215::IPRIOR215_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior215;
#[doc = "IPRIOR216 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior216::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior216::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior216`]
module"]
pub type IPRIOR216 = crate::Reg<iprior216::IPRIOR216_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior216;
#[doc = "IPRIOR217 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior217::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior217::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior217`]
module"]
pub type IPRIOR217 = crate::Reg<iprior217::IPRIOR217_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior217;
#[doc = "IPRIOR218 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior218::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior218::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior218`]
module"]
pub type IPRIOR218 = crate::Reg<iprior218::IPRIOR218_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior218;
#[doc = "IPRIOR219 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior219::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior219::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior219`]
module"]
pub type IPRIOR219 = crate::Reg<iprior219::IPRIOR219_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior219;
#[doc = "IPRIOR220 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior220::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior220::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior220`]
module"]
pub type IPRIOR220 = crate::Reg<iprior220::IPRIOR220_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior220;
#[doc = "IPRIOR221 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior221::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior221::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior221`]
module"]
pub type IPRIOR221 = crate::Reg<iprior221::IPRIOR221_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior221;
#[doc = "IPRIOR222 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior222::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior222::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior222`]
module"]
pub type IPRIOR222 = crate::Reg<iprior222::IPRIOR222_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior222;
#[doc = "IPRIOR223 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior223::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior223::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior223`]
module"]
pub type IPRIOR223 = crate::Reg<iprior223::IPRIOR223_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior223;
#[doc = "IPRIOR224 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior224::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior224::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior224`]
module"]
pub type IPRIOR224 = crate::Reg<iprior224::IPRIOR224_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior224;
#[doc = "IPRIOR225 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior225::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior225::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior225`]
module"]
pub type IPRIOR225 = crate::Reg<iprior225::IPRIOR225_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior225;
#[doc = "IPRIOR226 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior226::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior226::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior226`]
module"]
pub type IPRIOR226 = crate::Reg<iprior226::IPRIOR226_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior226;
#[doc = "IPRIOR227 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior227::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior227::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior227`]
module"]
pub type IPRIOR227 = crate::Reg<iprior227::IPRIOR227_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior227;
#[doc = "IPRIOR228 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior228::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior228::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior228`]
module"]
pub type IPRIOR228 = crate::Reg<iprior228::IPRIOR228_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior228;
#[doc = "IPRIOR229 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior229::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior229::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior229`]
module"]
pub type IPRIOR229 = crate::Reg<iprior229::IPRIOR229_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior229;
#[doc = "IPRIOR230 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior230::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior230::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior230`]
module"]
pub type IPRIOR230 = crate::Reg<iprior230::IPRIOR230_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior230;
#[doc = "IPRIOR231 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior231::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior231::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior231`]
module"]
pub type IPRIOR231 = crate::Reg<iprior231::IPRIOR231_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior231;
#[doc = "IPRIOR232 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior232::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior232::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior232`]
module"]
pub type IPRIOR232 = crate::Reg<iprior232::IPRIOR232_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior232;
#[doc = "IPRIOR233 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior233::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior233::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior233`]
module"]
pub type IPRIOR233 = crate::Reg<iprior233::IPRIOR233_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior233;
#[doc = "IPRIOR234 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior234::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior234::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior234`]
module"]
pub type IPRIOR234 = crate::Reg<iprior234::IPRIOR234_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior234;
#[doc = "IPRIOR235 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior235::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior235::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior235`]
module"]
pub type IPRIOR235 = crate::Reg<iprior235::IPRIOR235_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior235;
#[doc = "IPRIOR236 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior236::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior236::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior236`]
module"]
pub type IPRIOR236 = crate::Reg<iprior236::IPRIOR236_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior236;
#[doc = "IPRIOR237 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior237::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior237::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior237`]
module"]
pub type IPRIOR237 = crate::Reg<iprior237::IPRIOR237_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior237;
#[doc = "IPRIOR238 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior238::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior238::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior238`]
module"]
pub type IPRIOR238 = crate::Reg<iprior238::IPRIOR238_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior238;
#[doc = "IPRIOR239 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior239::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior239::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior239`]
module"]
pub type IPRIOR239 = crate::Reg<iprior239::IPRIOR239_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior239;
#[doc = "IPRIOR240 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior240::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior240::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior240`]
module"]
pub type IPRIOR240 = crate::Reg<iprior240::IPRIOR240_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior240;
#[doc = "IPRIOR241 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior241::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior241::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior241`]
module"]
pub type IPRIOR241 = crate::Reg<iprior241::IPRIOR241_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior241;
#[doc = "IPRIOR242 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior242::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior242::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior242`]
module"]
pub type IPRIOR242 = crate::Reg<iprior242::IPRIOR242_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior242;
#[doc = "IPRIOR243 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior243::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior243::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior243`]
module"]
pub type IPRIOR243 = crate::Reg<iprior243::IPRIOR243_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior243;
#[doc = "IPRIOR244 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior244::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior244::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior244`]
module"]
pub type IPRIOR244 = crate::Reg<iprior244::IPRIOR244_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior244;
#[doc = "IPRIOR245 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior245::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior245::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior245`]
module"]
pub type IPRIOR245 = crate::Reg<iprior245::IPRIOR245_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior245;
#[doc = "IPRIOR246 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior246::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior246::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior246`]
module"]
pub type IPRIOR246 = crate::Reg<iprior246::IPRIOR246_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior246;
#[doc = "IPRIOR247 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior247::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior247::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior247`]
module"]
pub type IPRIOR247 = crate::Reg<iprior247::IPRIOR247_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior247;
#[doc = "IPRIOR248 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior248::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior248::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior248`]
module"]
pub type IPRIOR248 = crate::Reg<iprior248::IPRIOR248_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior248;
#[doc = "IPRIOR249 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior249::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior249::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior249`]
module"]
pub type IPRIOR249 = crate::Reg<iprior249::IPRIOR249_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior249;
#[doc = "IPRIOR250 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior250::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior250::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior250`]
module"]
pub type IPRIOR250 = crate::Reg<iprior250::IPRIOR250_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior250;
#[doc = "IPRIOR251 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior251::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior251::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior251`]
module"]
pub type IPRIOR251 = crate::Reg<iprior251::IPRIOR251_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior251;
#[doc = "IPRIOR252 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior252::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior252::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior252`]
module"]
pub type IPRIOR252 = crate::Reg<iprior252::IPRIOR252_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior252;
#[doc = "IPRIOR253 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior253::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior253::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior253`]
module"]
pub type IPRIOR253 = crate::Reg<iprior253::IPRIOR253_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior253;
#[doc = "IPRIOR254 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior254::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior254::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior254`]
module"]
pub type IPRIOR254 = crate::Reg<iprior254::IPRIOR254_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior254;
#[doc = "IPRIOR255 (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior255::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior255::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior255`]
module"]
pub type IPRIOR255 = crate::Reg<iprior255::IPRIOR255_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior255;
#[doc = "SCTLR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctlr`]
module"]
pub type SCTLR = crate::Reg<sctlr::SCTLR_SPEC>;
#[doc = "System Control Register"]
pub mod sctlr;
#[doc = "STK_CTLR (rw) register accessor: System counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_ctlr`]
module"]
pub type STK_CTLR = crate::Reg<stk_ctlr::STK_CTLR_SPEC>;
#[doc = "System counter control register"]
pub mod stk_ctlr;
#[doc = "STK_SR (rw) register accessor: System START\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_sr`]
module"]
pub type STK_SR = crate::Reg<stk_sr::STK_SR_SPEC>;
#[doc = "System START"]
pub mod stk_sr;
#[doc = "STK_CNTL (rw) register accessor: System counter low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cntl`]
module"]
pub type STK_CNTL = crate::Reg<stk_cntl::STK_CNTL_SPEC>;
#[doc = "System counter low register"]
pub mod stk_cntl;
#[doc = "STK_CNTH (rw) register accessor: System counter high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cnth`]
module"]
pub type STK_CNTH = crate::Reg<stk_cnth::STK_CNTH_SPEC>;
#[doc = "System counter high register"]
pub mod stk_cnth;
#[doc = "STK_CMPLR (rw) register accessor: System compare low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cmplr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cmplr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cmplr`]
module"]
pub type STK_CMPLR = crate::Reg<stk_cmplr::STK_CMPLR_SPEC>;
#[doc = "System compare low register"]
pub mod stk_cmplr;
#[doc = "STK_CMPHR (rw) register accessor: System compare high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cmphr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cmphr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cmphr`]
module"]
pub type STK_CMPHR = crate::Reg<stk_cmphr::STK_CMPHR_SPEC>;
#[doc = "System compare high register"]
pub mod stk_cmphr;
