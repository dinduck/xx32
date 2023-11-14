#[doc = "Register `RXMIR1` reader"]
pub type R = crate::R<RXMIR1_SPEC>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `IDE` reader - Identifier extension"]
pub type IDE_R = crate::BitReader;
#[doc = "Field `EXID` reader - extended identifier"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `STID` reader - Standard identifier"]
pub type STID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier extension"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - extended identifier"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Standard identifier"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "CAN receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmir1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMIR1_SPEC;
impl crate::RegisterSpec for RXMIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmir1::R`](R) reader structure"]
impl crate::Readable for RXMIR1_SPEC {}
#[doc = "`reset()` method sets RXMIR1 to value 0"]
impl crate::Resettable for RXMIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
