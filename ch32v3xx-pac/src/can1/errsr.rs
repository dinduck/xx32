#[doc = "Register `ERRSR` reader"]
pub type R = crate::R<ERRSR_SPEC>;
#[doc = "Register `ERRSR` writer"]
pub type W = crate::W<ERRSR_SPEC>;
#[doc = "Field `EWGF` reader - Error warning flag"]
pub type EWGF_R = crate::BitReader;
#[doc = "Field `EPVF` reader - Error passive flag"]
pub type EPVF_R = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus-off flag"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `LEC` reader - Last error code"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Last error code"]
pub type LEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TEC` reader - Least significant byte of the 9-bit transmit error counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Error warning flag"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error passive flag"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off flag"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Last error code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Least significant byte of the 9-bit transmit error counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive error counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Last error code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<ERRSR_SPEC, 4> {
        LEC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRSR_SPEC;
impl crate::RegisterSpec for ERRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errsr::R`](R) reader structure"]
impl crate::Readable for ERRSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`errsr::W`](W) writer structure"]
impl crate::Writable for ERRSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRSR to value 0"]
impl crate::Resettable for ERRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
