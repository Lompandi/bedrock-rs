use crate::version::v748::types::ItemStackResponseContainerInfo;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ItemStackNetResult {
    Success(
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        Vec<ItemStackResponseContainerInfo>,
    ) = 0,
    Error = 1,
    InvalidRequestActionType = 2,
    ActionRequestNotAllowed = 3,
    ScreenHandlerEndRequestFailed = 4,
    ItemRequestActionHandlerCommitFailed = 5,
    InvalidRequestCraftActionType = 6,
    InvalidCraftRequest = 7,
    InvalidCraftRequestScreen = 8,
    InvalidCraftResult = 9,
    InvalidCraftResultIndex = 10,
    InvalidCraftResultItem = 11,
    InvalidItemNetId = 12,
    MissingCreatedOutputContainer = 13,
    FailedToSetCreatedItemOutputSlot = 14,
    RequestAlreadyInProgress = 15,
    FailedToInitSparseContainer = 16,
    ResultTransferFailed = 17,
    ExpectedItemSlotNotFullyConsumed = 18,
    ExpectedAnywhereItemNotFullyConsumed = 19,
    ItemAlreadyConsumedFromSlot = 20,
    ConsumedTooMuchFromSlot = 21,
    MismatchSlotExpectedConsumedItem = 22,
    MismatchSlotExpectedConsumedItemNetIdVariant = 23,
    FailedToMatchExpectedSlotConsumedItem = 24,
    FailedToMatchExpectedAllowedAnywhereConsumedItem = 25,
    ConsumedItemOutOfAllowedSlotRange = 26,
    ConsumedItemNotAllowed = 27,
    PlayerNotInCreativeMode = 28,
    InvalidExperimentalRecipeRequest = 29,
    FailedToCraftCreative = 30,
    FailedToGetLevelRecipe = 31,
    FailedToFindRecipeByNetId = 32,
    MismatchedCraftingSize = 33,
    MissingInputSparseContainer = 34,
    MismatchedRecipeForInputGridItems = 35,
    EmptyCraftResults = 36,
    FailedToEnchant = 37,
    MissingInputItem = 38,
    InsufficientPlayerLevelToEnchant = 39,
    MissingMaterialItem = 40,
    MissingActor = 41,
    UnknownPrimaryEffect = 42,
    PrimaryEffectOutOfRange = 43,
    PrimaryEffectUnavailable = 44,
    SecondaryEffectOutOfRange = 45,
    SecondaryEffectUnavailable = 46,
    DstContainerEqualToCreatedOutputContainer = 47,
    DstContainerAndSlotEqualToSrcContainerAndSlot = 48,
    FailedToValidateSrcSlot = 49,
    FailedToValidateDstSlot = 50,
    InvalidAdjustedAmount = 51,
    InvalidItemSetType = 52,
    InvalidTransferAmount = 53,
    CannotSwapItem = 54,
    CannotPlaceItem = 55,
    UnhandledItemSetType = 56,
    InvalidRemovedAmount = 57,
    InvalidRegion = 58,
    CannotDropItem = 59,
    CannotDestroyItem = 60,
    InvalidSourceContainer = 61,
    ItemNotConsumed = 62,
    InvalidNumCrafts = 63,
    InvalidCraftResultStackSize = 64,
    CannotRemoveItem = 65,
    CannotConsumeItem = 66,
    ScreenStackError = 67,
}