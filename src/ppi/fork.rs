#[doc = "Description cluster\\[0\\]: Channel 0 task end-point\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tep](tep) module"]
pub type TEP = crate::Reg<u32, _TEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEP;
#[doc = "`read()` method returns [tep::R](tep::R) reader structure"]
impl crate::Readable for TEP {}
#[doc = "`write(|w| ..)` method takes [tep::W](tep::W) writer structure"]
impl crate::Writable for TEP {}
#[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
pub mod tep;
