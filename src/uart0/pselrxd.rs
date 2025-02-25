#[doc = "Reader of register PSELRXD"]
pub type R = crate::R<u32, super::PSELRXD>;
#[doc = "Writer for register PSELRXD"]
pub type W = crate::W<u32, super::PSELRXD>;
#[doc = "Register PSELRXD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELRXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for UART RXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELRXD_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED,
}
impl From<PSELRXD_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELRXD_A) -> Self {
        match variant {
            PSELRXD_A::DISCONNECTED => 4294967295,
        }
    }
}
#[doc = "Reader of field `PSELRXD`"]
pub type PSELRXD_R = crate::R<u32, PSELRXD_A>;
impl PSELRXD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELRXD_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELRXD_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELRXD_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELRXD`"]
pub struct PSELRXD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELRXD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELRXD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELRXD_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    pub fn pselrxd(&self) -> PSELRXD_R {
        PSELRXD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    pub fn pselrxd(&mut self) -> PSELRXD_W {
        PSELRXD_W { w: self }
    }
}
