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
#[doc = "Write '1' to Enable interrupt for DATARDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<DATARDY_A> for bool {
    #[inline(always)]
    fn from(variant: DATARDY_A) -> Self {
        match variant {
            DATARDY_A::DISABLED => false,
            DATARDY_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DATARDY`"]
pub type DATARDY_R = crate::R<bool, DATARDY_A>;
impl DATARDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATARDY_A {
        match self.bits {
            false => DATARDY_A::DISABLED,
            true => DATARDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DATARDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DATARDY_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for DATARDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARDY_AW {
    #[doc = "1: Enable"]
    SET,
}
impl From<DATARDY_AW> for bool {
    #[inline(always)]
    fn from(variant: DATARDY_AW) -> Self {
        match variant {
            DATARDY_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `DATARDY`"]
pub struct DATARDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATARDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DATARDY_AW::SET)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for DATARDY event"]
    #[inline(always)]
    pub fn datardy(&self) -> DATARDY_R {
        DATARDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for DATARDY event"]
    #[inline(always)]
    pub fn datardy(&mut self) -> DATARDY_W {
        DATARDY_W { w: self }
    }
}
