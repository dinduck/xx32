#[doc = "Register `FNR` reader"]
pub type R = crate::R<FNR_SPEC>;
#[doc = "Field `FN` reader - Frame number"]
pub type FN_R = crate::FieldReader<u16>;
#[doc = "Field `LSOF` reader - Lost SOF"]
pub type LSOF_R = crate::FieldReader;
#[doc = "Field `LCK` reader - Locked"]
pub type LCK_R = crate::BitReader;
#[doc = "Field `RXDM` reader - Receive data - line status"]
pub type RXDM_R = crate::BitReader;
#[doc = "Field `RXDP` reader - Receive data + line status"]
pub type RXDP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 11:12 - Lost SOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Locked"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FNR_SPEC;
impl crate::RegisterSpec for FNR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fnr::R`](R) reader structure"]
impl crate::Readable for FNR_SPEC {}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
