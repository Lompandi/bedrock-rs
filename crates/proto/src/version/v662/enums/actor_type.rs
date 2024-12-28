use bedrockrs_macros::ProtoCodec;

mod flags {
    pub const UNDEFINED: i32 = 1;
    pub const TYPE_MASK: i32 = 0x000000ff;
    pub const MOB: i32 = 0x00000100;
    pub const PATHFINDER_MOB: i32 = 0x00000200 | MOB;
    pub const MONSTER: i32 = 0x00000800 | PATHFINDER_MOB;
    pub const ANIMAL: i32 = 0x00001000 | PATHFINDER_MOB;
    pub const TAMABLE_ANIMAL: i32 = 0x00004000 | ANIMAL;
    pub const AMBIENT: i32 = 0x00008000 | MOB;
    pub const UNDEAD_MOB: i32 = 0x00010000 | MONSTER;
    pub const ZOMBIE_MONSTER: i32 = 0x00020000 | UNDEAD_MOB;
    pub const ARTHROPOD: i32 = 0x00040000 | MONSTER;
    pub const MINECART: i32 = 0x00080000;
    pub const SKELETONMONSTER: i32 = 0x00100000 | UNDEAD_MOB;
    pub const EQUINEANIMAL: i32 = 0x00200000 | TAMABLE_ANIMAL;
    pub const PROJECTILE: i32 = 0x00400000;
    pub const ABSTRACTARROW: i32 = 0x00800000;
    pub const WATERANIMAL: i32 = 0x00002000 | PATHFINDER_MOB;
    pub const VILLAGERBASE: i32 = 0x01000000 | PATHFINDER_MOB;
    pub const CHICKEN: i32 = 10 | ANIMAL;
    pub const COW: i32 = 11 | ANIMAL;
    pub const PIG: i32 = 12 | ANIMAL;
    pub const SHEEP: i32 = 13 | ANIMAL;
    pub const WOLF: i32 = 14 | TAMABLE_ANIMAL;
    pub const VILLAGER: i32 = 15 | VILLAGERBASE;
    pub const MUSHROOMCOW: i32 = 16 | ANIMAL;
    pub const SQUID: i32 = 17 | WATERANIMAL;
    pub const RABBIT: i32 = 18 | ANIMAL;
    pub const BAT: i32 = 19 | AMBIENT;
    pub const IRONGOLEM: i32 = 20 | PATHFINDER_MOB;
    pub const SNOWGOLEM: i32 = 21 | PATHFINDER_MOB;
    pub const OCELOT: i32 = 22 | TAMABLE_ANIMAL;
    pub const HORSE: i32 = 23 | EQUINEANIMAL;
    pub const POLARBEAR: i32 = 28 | ANIMAL;
    pub const LLAMA: i32 = 29 | ANIMAL;
    pub const PARROT: i32 = 30 | TAMABLE_ANIMAL;
    pub const DOLPHIN: i32 = 31 | WATERANIMAL;
    pub const DONKEY: i32 = 24 | EQUINEANIMAL;
    pub const MULE: i32 = 25 | EQUINEANIMAL;
    pub const SKELETONHORSE: i32 = 26 | EQUINEANIMAL | UNDEAD_MOB;
    pub const ZOMBIEHORSE: i32 = 27 | EQUINEANIMAL | UNDEAD_MOB;
    pub const ZOMBIE: i32 = 32 | ZOMBIE_MONSTER;
    pub const CREEPER: i32 = 33 | MONSTER;
    pub const SKELETON: i32 = 34 | SKELETONMONSTER;
    pub const SPIDER: i32 = 35 | ARTHROPOD;
    pub const PIGZOMBIE: i32 = 36 | UNDEAD_MOB;
    pub const SLIME: i32 = 37 | MONSTER;
    pub const ENDERMAN: i32 = 38 | MONSTER;
    pub const SILVERFISH: i32 = 39 | ARTHROPOD;
    pub const CAVESPIDER: i32 = 40 | ARTHROPOD;
    pub const GHAST: i32 = 41 | MONSTER;
    pub const LAVASLIME: i32 = 42 | MONSTER;
    pub const BLAZE: i32 = 43 | MONSTER;
    pub const ZOMBIEVILLAGER: i32 = 44 | ZOMBIE_MONSTER;
    pub const WITCH: i32 = 45 | MONSTER;
    pub const STRAY: i32 = 46 | SKELETONMONSTER;
    pub const HUSK: i32 = 47 | ZOMBIE_MONSTER;
    pub const WITHERSKELETON: i32 = 48 | SKELETONMONSTER;
    pub const GUARDIAN: i32 = 49 | MONSTER;
    pub const ELDERGUARDIAN: i32 = 50 | MONSTER;
    pub const NPC: i32 = 51 | MOB;
    pub const WITHERBOSS: i32 = 52 | UNDEAD_MOB;
    pub const DRAGON: i32 = 53 | MONSTER;
    pub const SHULKER: i32 = 54 | MONSTER;
    pub const ENDERMITE: i32 = 55 | ARTHROPOD;
    pub const AGENT: i32 = 56 | MOB;
    pub const VINDICATOR: i32 = 57 | MONSTER;
    pub const PHANTOM: i32 = 58 | UNDEAD_MOB;
    pub const ILLAGERBEAST: i32 = 59 | MONSTER;
    pub const ARMORSTAND: i32 = 61 | MOB;
    pub const TRIPODCAMERA: i32 = 62 | MOB;
    pub const PLAYER: i32 = 63 | MOB;
    pub const ITEMENTITY: i32 = 64;
    pub const PRIMEDTNT: i32 = 65;
    pub const FALLINGBLOCK: i32 = 66;
    pub const MOVINGBLOCK: i32 = 67;
    pub const EXPERIENCEPOTION: i32 = 68 | PROJECTILE;
    pub const EXPERIENCE: i32 = 69;
    pub const EYEOFENDER: i32 = 70;
    pub const ENDERCRYSTAL: i32 = 71;
    pub const FIREWORKSROCKET: i32 = 72;
    pub const TRIDENT: i32 = 73 | PROJECTILE | ABSTRACTARROW;
    pub const TURTLE: i32 = 74 | ANIMAL;
    pub const CAT: i32 = 75 | TAMABLE_ANIMAL;
    pub const SHULKERBULLET: i32 = 76 | PROJECTILE;
    pub const FISHINGHOOK: i32 = 77;
    pub const CHALKBOARD: i32 = 78;
    pub const DRAGONFIREBALL: i32 = 79 | PROJECTILE;
    pub const ARROW: i32 = 80 | PROJECTILE | ABSTRACTARROW;
    pub const SNOWBALL: i32 = 81 | PROJECTILE;
    pub const THROWNEGG: i32 = 82 | PROJECTILE;
    pub const PAINTING: i32 = 83;
    pub const LARGEFIREBALL: i32 = 85 | PROJECTILE;
    pub const THROWNPOTION: i32 = 86 | PROJECTILE;
    pub const ENDERPEARL: i32 = 87 | PROJECTILE;
    pub const LEASHKNOT: i32 = 88;
    pub const WITHERSKULL: i32 = 89 | PROJECTILE;
    pub const BOATRIDEABLE: i32 = 90;
    pub const WITHERSKULLDANGEROUS: i32 = 91 | PROJECTILE;
    pub const LIGHTNINGBOLT: i32 = 93;
    pub const SMALLFIREBALL: i32 = 94 | PROJECTILE;
    pub const AREAEFFECTCLOUD: i32 = 95;
    pub const LINGERINGPOTION: i32 = 101 | PROJECTILE;
    pub const LLAMASPIT: i32 = 102 | PROJECTILE;
    pub const EVOCATIONFANG: i32 = 103 | PROJECTILE;
    pub const EVOCATIONILLAGER: i32 = 104 | MONSTER;
    pub const VEX: i32 = 105 | MONSTER;
    pub const MINECARTRIDEABLE: i32 = 84 | MINECART;
    pub const MINECARTHOPPER: i32 = 96 | MINECART;
    pub const MINECARTTNT: i32 = 97 | MINECART;
    pub const MINECARTCHEST: i32 = 98 | MINECART;
    pub const MINECARTFURNACE: i32 = 99 | MINECART;
    pub const MINECARTCOMMANDBLOCK: i32 = 100 | MINECART;
    pub const ICEBOMB: i32 = 106 | PROJECTILE;
    pub const BALLOON: i32 = 107;
    pub const PUFFERFISH: i32 = 108 | WATERANIMAL;
    pub const SALMON: i32 = 109 | WATERANIMAL;
    pub const DROWNED: i32 = 110 | ZOMBIE_MONSTER;
    pub const TROPICALFISH: i32 = 111 | WATERANIMAL;
    pub const FISH: i32 = 112 | WATERANIMAL;
    pub const PANDA: i32 = 113 | ANIMAL;
    pub const PILLAGER: i32 = 114 | MONSTER;
    pub const VILLAGERV2: i32 = 115 | VILLAGERBASE;
    pub const ZOMBIEVILLAGERV2: i32 = 116 | ZOMBIE_MONSTER;
    pub const SHIELD: i32 = 117;
    pub const WANDERINGTRADER: i32 = 118 | PATHFINDER_MOB;
    pub const LECTERN: i32 = 119;
    pub const ELDERGUARDIANGHOST: i32 = 120 | MONSTER;
    pub const FOX: i32 = 121 | ANIMAL;
    pub const BEE: i32 = 122 | MOB;
    pub const PIGLIN: i32 = 123 | MOB;
    pub const HOGLIN: i32 = 124 | ANIMAL;
    pub const STRIDER: i32 = 125 | ANIMAL;
    pub const ZOGLIN: i32 = 126 | UNDEAD_MOB;
    pub const PIGLINBRUTE: i32 = 127 | MOB;
    pub const GOAT: i32 = 128 | ANIMAL;
    pub const GLOWSQUID: i32 = 129 | WATERANIMAL;
    pub const AXOLOTL: i32 = 130 | ANIMAL;
    pub const WARDEN: i32 = 131 | MONSTER;
    pub const FROG: i32 = 132 | ANIMAL;
    pub const TADPOLE: i32 = 133 | WATERANIMAL;
    pub const ALLAY: i32 = 134 | MOB;
    pub const CHESTBOATRIDEABLE: i32 = 136 | BOATRIDEABLE;
    pub const TRADERLLAMA: i32 = 137 | LLAMA;
    pub const CAMEL: i32 = 138 | ANIMAL;
    pub const SNIFFER: i32 = 139 | ANIMAL;
    pub const BREEZE: i32 = 140 | MONSTER;
    pub const BREEZEWINDCHARGEPROJECTILE: i32 = 141 | PROJECTILE;
    pub const ARMADILLO: i32 = 142 | ANIMAL;
    pub const WINDCHARGEPROJECTILE: i32 = 143 | PROJECTILE;
    pub const BOGGED: i32 = 144 | SKELETONMONSTER;
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
pub enum ActorType {
    Undefined = flags::UNDEFINED,
    TypeMask = flags::TYPE_MASK,
    Mob = flags::MOB,
    PathfinderMob = flags::PATHFINDER_MOB,
    Monster = flags::MONSTER,
    Animal = flags::ANIMAL,
    TamableAnimal = flags::TAMABLE_ANIMAL,
    Ambient = flags::AMBIENT,
    UndeadMob = flags::UNDEAD_MOB,
    ZombieMonster = flags::ZOMBIE_MONSTER,
    Arthropod = flags::ARTHROPOD,
    Minecart = flags::MINECART,
    SkeletonMonster = flags::SKELETONMONSTER,
    EquineAnimal = flags::EQUINEANIMAL,
    Projectile = flags::PROJECTILE,
    AbstractArrow = flags::ABSTRACTARROW,
    WaterAnimal = flags::WATERANIMAL,
    VillagerBase = flags::VILLAGERBASE,
    Chicken = flags::CHICKEN,
    Cow = flags::COW,
    Pig = flags::PIG,
    Sheep = flags::SHEEP,
    Wolf = flags::WOLF,
    Villager = flags::VILLAGER,
    MushroomCow = flags::MUSHROOMCOW,
    Squid = flags::SQUID,
    Rabbit = flags::RABBIT,
    Bat = flags::BAT,
    IronGolem = flags::IRONGOLEM,
    SnowGolem = flags::SNOWGOLEM,
    Ocelot = flags::OCELOT,
    Horse = flags::HORSE,
    PolarBear = flags::POLARBEAR,
    Llama = flags::LLAMA,
    Parrot = flags::PARROT,
    Dolphin = flags::DOLPHIN,
    Donkey = flags::DONKEY,
    Mule = flags::MULE,
    SkeletonHorse = flags::SKELETONHORSE,
    ZombieHorse = flags::ZOMBIEHORSE,
    Zombie = flags::ZOMBIE,
    Creeper = flags::CREEPER,
    Skeleton = flags::SKELETON,
    Spider = flags::SPIDER,
    PigZombie = flags::PIGZOMBIE,
    Slime = flags::SLIME,
    EnderMan = flags::ENDERMAN,
    Silverfish = flags::SILVERFISH,
    CaveSpider = flags::CAVESPIDER,
    Ghast = flags::GHAST,
    LavaSlime = flags::LAVASLIME,
    Blaze = flags::BLAZE,
    ZombieVillager = flags::ZOMBIEVILLAGER,
    Witch = flags::WITCH,
    Stray = flags::STRAY,
    Husk = flags::HUSK,
    WitherSkeleton = flags::WITHERSKELETON,
    Guardian = flags::GUARDIAN,
    ElderGuardian = flags::ELDERGUARDIAN,
    Npc = flags::NPC,
    WitherBoss = flags::WITHERBOSS,
    Dragon = flags::DRAGON,
    Shulker = flags::SHULKER,
    Endermite = flags::ENDERMITE,
    Agent = flags::AGENT,
    Vindicator = flags::VINDICATOR,
    Phantom = flags::PHANTOM,
    IllagerBeast = flags::ILLAGERBEAST,
    ArmorStand = flags::ARMORSTAND,
    TripodCamera = flags::TRIPODCAMERA,
    Player = flags::PLAYER,
    ItemEntity = flags::ITEMENTITY,
    PrimedTnt = flags::PRIMEDTNT,
    FallingBlock = flags::FALLINGBLOCK,
    MovingBlock = flags::MOVINGBLOCK,
    ExperiencePotion = flags::EXPERIENCEPOTION,
    Experience = flags::EXPERIENCE,
    EyeOfEnder = flags::EYEOFENDER,
    EnderCrystal = flags::ENDERCRYSTAL,
    FireworksRocket = flags::FIREWORKSROCKET,
    Trident = flags::TRIDENT,
    Turtle = flags::TURTLE,
    Cat = flags::CAT,
    ShulkerBullet = flags::SHULKERBULLET,
    FishingHook = flags::FISHINGHOOK,
    Chalkboard = flags::CHALKBOARD,
    DragonFireball = flags::DRAGONFIREBALL,
    Arrow = flags::ARROW,
    Snowball = flags::SNOWBALL,
    ThrownEgg = flags::THROWNEGG,
    Painting = flags::PAINTING,
    LargeFireball = flags::LARGEFIREBALL,
    ThrownPotion = flags::THROWNPOTION,
    Enderpearl = flags::ENDERPEARL,
    LeashKnot = flags::LEASHKNOT,
    WitherSkull = flags::WITHERSKULL,
    BoatRideable = flags::BOATRIDEABLE,
    WitherSkullDangerous = flags::WITHERSKULLDANGEROUS,
    LightningBolt = flags::LIGHTNINGBOLT,
    SmallFireball = flags::SMALLFIREBALL,
    AreaEffectCloud = flags::AREAEFFECTCLOUD,
    LingeringPotion = flags::LINGERINGPOTION,
    LlamaSpit = flags::LLAMASPIT,
    EvocationFang = flags::EVOCATIONFANG,
    EvocationIllager = flags::EVOCATIONILLAGER,
    Vex = flags::VEX,
    MinecartRideable = flags::MINECARTRIDEABLE,
    MinecartHopper = flags::MINECARTHOPPER,
    MinecartTNT = flags::MINECARTTNT,
    MinecartChest = flags::MINECARTCHEST,
    MinecartFurnace = flags::MINECARTFURNACE,
    MinecartCommandBlock = flags::MINECARTCOMMANDBLOCK,
    IceBomb = flags::ICEBOMB,
    Balloon = flags::BALLOON,
    Pufferfish = flags::PUFFERFISH,
    Salmon = flags::SALMON,
    Drowned = flags::DROWNED,
    Tropicalfish = flags::TROPICALFISH,
    Fish = flags::FISH,
    Panda = flags::PANDA,
    Pillager = flags::PILLAGER,
    VillagerV2 = flags::VILLAGERV2,
    ZombieVillagerV2 = flags::ZOMBIEVILLAGERV2,
    Shield = flags::SHIELD,
    WanderingTrader = flags::WANDERINGTRADER,
    Lectern = flags::LECTERN,
    ElderGuardianGhost = flags::ELDERGUARDIANGHOST,
    Fox = flags::FOX,
    Bee = flags::BEE,
    Piglin = flags::PIGLIN,
    Hoglin = flags::HOGLIN,
    Strider = flags::STRIDER,
    Zoglin = flags::ZOGLIN,
    PiglinBrute = flags::PIGLINBRUTE,
    Goat = flags::GOAT,
    GlowSquid = flags::GLOWSQUID,
    Axolotl = flags::AXOLOTL,
    Warden = flags::WARDEN,
    Frog = flags::FROG,
    Tadpole = flags::TADPOLE,
    Allay = flags::ALLAY,
    ChestBoatRideable = flags::CHESTBOATRIDEABLE,
    TraderLlama = flags::TRADERLLAMA,
    Camel = flags::CAMEL,
    Sniffer = flags::SNIFFER,
    Breeze = flags::BREEZE,
    BreezeWindChargeProjectile = flags::BREEZEWINDCHARGEPROJECTILE,
    Armadillo = flags::ARMADILLO,
    WindChargeProjectile = flags::WINDCHARGEPROJECTILE,
    Bogged = flags::BOGGED,
}
