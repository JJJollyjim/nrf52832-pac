#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between LASTTX event and STARTRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<LASTTX_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_STARTRX_A) -> Self {
        match variant {
            LASTTX_STARTRX_A::DISABLED => false,
            LASTTX_STARTRX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LASTTX_STARTRX`"]
pub type LASTTX_STARTRX_R = crate::R<bool, LASTTX_STARTRX_A>;
impl LASTTX_STARTRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_STARTRX_A {
        match self.bits {
            false => LASTTX_STARTRX_A::DISABLED,
            true => LASTTX_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LASTTX_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LASTTX_STARTRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `LASTTX_STARTRX`"]
pub struct LASTTX_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_STARTRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_STARTRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Shortcut between LASTTX event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<LASTTX_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_SUSPEND_A) -> Self {
        match variant {
            LASTTX_SUSPEND_A::DISABLED => false,
            LASTTX_SUSPEND_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LASTTX_SUSPEND`"]
pub type LASTTX_SUSPEND_R = crate::R<bool, LASTTX_SUSPEND_A>;
impl LASTTX_SUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_SUSPEND_A {
        match self.bits {
            false => LASTTX_SUSPEND_A::DISABLED,
            true => LASTTX_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LASTTX_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LASTTX_SUSPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `LASTTX_SUSPEND`"]
pub struct LASTTX_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_SUSPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_SUSPEND_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Shortcut between LASTTX event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<LASTTX_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_STOP_A) -> Self {
        match variant {
            LASTTX_STOP_A::DISABLED => false,
            LASTTX_STOP_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LASTTX_STOP`"]
pub type LASTTX_STOP_R = crate::R<bool, LASTTX_STOP_A>;
impl LASTTX_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_STOP_A {
        match self.bits {
            false => LASTTX_STOP_A::DISABLED,
            true => LASTTX_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LASTTX_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LASTTX_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `LASTTX_STOP`"]
pub struct LASTTX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_STOP_A::ENABLED)
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
#[doc = "Shortcut between LASTRX event and STARTTX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STARTTX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<LASTRX_STARTTX_A> for bool {
    #[inline(always)]
    fn from(variant: LASTRX_STARTTX_A) -> Self {
        match variant {
            LASTRX_STARTTX_A::DISABLED => false,
            LASTRX_STARTTX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LASTRX_STARTTX`"]
pub type LASTRX_STARTTX_R = crate::R<bool, LASTRX_STARTTX_A>;
impl LASTRX_STARTTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTRX_STARTTX_A {
        match self.bits {
            false => LASTRX_STARTTX_A::DISABLED,
            true => LASTRX_STARTTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LASTRX_STARTTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LASTRX_STARTTX_A::ENABLED
    }
}
#[doc = "Write proxy for field `LASTRX_STARTTX`"]
pub struct LASTRX_STARTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTRX_STARTTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTRX_STARTTX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTRX_STARTTX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTRX_STARTTX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Shortcut between LASTRX event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<LASTRX_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LASTRX_STOP_A) -> Self {
        match variant {
            LASTRX_STOP_A::DISABLED => false,
            LASTRX_STOP_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LASTRX_STOP`"]
pub type LASTRX_STOP_R = crate::R<bool, LASTRX_STOP_A>;
impl LASTRX_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTRX_STOP_A {
        match self.bits {
            false => LASTRX_STOP_A::DISABLED,
            true => LASTRX_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LASTRX_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LASTRX_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `LASTRX_STOP`"]
pub struct LASTRX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTRX_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTRX_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTRX_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTRX_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Shortcut between LASTTX event and STARTRX task"]
    #[inline(always)]
    pub fn lasttx_startrx(&self) -> LASTTX_STARTRX_R {
        LASTTX_STARTRX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between LASTTX event and SUSPEND task"]
    #[inline(always)]
    pub fn lasttx_suspend(&self) -> LASTTX_SUSPEND_R {
        LASTTX_SUSPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Shortcut between LASTTX event and STOP task"]
    #[inline(always)]
    pub fn lasttx_stop(&self) -> LASTTX_STOP_R {
        LASTTX_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Shortcut between LASTRX event and STARTTX task"]
    #[inline(always)]
    pub fn lastrx_starttx(&self) -> LASTRX_STARTTX_R {
        LASTRX_STARTTX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Shortcut between LASTRX event and STOP task"]
    #[inline(always)]
    pub fn lastrx_stop(&self) -> LASTRX_STOP_R {
        LASTRX_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Shortcut between LASTTX event and STARTRX task"]
    #[inline(always)]
    pub fn lasttx_startrx(&mut self) -> LASTTX_STARTRX_W {
        LASTTX_STARTRX_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between LASTTX event and SUSPEND task"]
    #[inline(always)]
    pub fn lasttx_suspend(&mut self) -> LASTTX_SUSPEND_W {
        LASTTX_SUSPEND_W { w: self }
    }
    #[doc = "Bit 9 - Shortcut between LASTTX event and STOP task"]
    #[inline(always)]
    pub fn lasttx_stop(&mut self) -> LASTTX_STOP_W {
        LASTTX_STOP_W { w: self }
    }
    #[doc = "Bit 10 - Shortcut between LASTRX event and STARTTX task"]
    #[inline(always)]
    pub fn lastrx_starttx(&mut self) -> LASTRX_STARTTX_W {
        LASTRX_STARTTX_W { w: self }
    }
    #[doc = "Bit 12 - Shortcut between LASTRX event and STOP task"]
    #[inline(always)]
    pub fn lastrx_stop(&mut self) -> LASTRX_STOP_W {
        LASTRX_STOP_W { w: self }
    }
}
