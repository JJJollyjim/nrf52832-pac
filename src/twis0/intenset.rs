#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to Enable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        match variant {
            STOPPED_A::DISABLED => false,
            STOPPED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `STOPPED`"]
pub type STOPPED_R = crate::R<bool, STOPPED_A>;
impl STOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        match variant {
            STOPPED_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `STOPPED`"]
pub struct STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STOPPED_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        match variant {
            ERROR_A::DISABLED => false,
            ERROR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        match variant {
            ERROR_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for RXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<RXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_A) -> Self {
        match variant {
            RXSTARTED_A::DISABLED => false,
            RXSTARTED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `RXSTARTED`"]
pub type RXSTARTED_R = crate::R<bool, RXSTARTED_A>;
impl RXSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTARTED_A {
        match self.bits {
            false => RXSTARTED_A::DISABLED,
            true => RXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<RXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_AW) -> Self {
        match variant {
            RXSTARTED_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `RXSTARTED`"]
pub struct RXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXSTARTED_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for TXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<TXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_A) -> Self {
        match variant {
            TXSTARTED_A::DISABLED => false,
            TXSTARTED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TXSTARTED`"]
pub type TXSTARTED_R = crate::R<bool, TXSTARTED_A>;
impl TXSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTARTED_A {
        match self.bits {
            false => TXSTARTED_A::DISABLED,
            true => TXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for TXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<TXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_AW) -> Self {
        match variant {
            TXSTARTED_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `TXSTARTED`"]
pub struct TXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXSTARTED_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for WRITE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        match variant {
            WRITE_A::DISABLED => false,
            WRITE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, WRITE_A>;
impl WRITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::DISABLED,
            true => WRITE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for WRITE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: WRITE_AW) -> Self {
        match variant {
            WRITE_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(WRITE_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for READ event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        match variant {
            READ_A::DISABLED => false,
            READ_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, READ_A>;
impl READ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::DISABLED,
            true => READ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for READ event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<READ_AW> for bool {
    #[inline(always)]
    fn from(variant: READ_AW) -> Self {
        match variant {
            READ_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(READ_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to Enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RXSTARTED_R {
        RXSTARTED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&self) -> TXSTARTED_R {
        TXSTARTED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write '1' to Enable interrupt for WRITE event"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write '1' to Enable interrupt for READ event"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to Enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&mut self) -> RXSTARTED_W {
        RXSTARTED_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&mut self) -> TXSTARTED_W {
        TXSTARTED_W { w: self }
    }
    #[doc = "Bit 25 - Write '1' to Enable interrupt for WRITE event"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 26 - Write '1' to Enable interrupt for READ event"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
}
