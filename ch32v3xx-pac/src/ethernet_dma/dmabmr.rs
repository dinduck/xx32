#[doc = "Register `DMABMR` reader"]
pub type R = crate::R<DMABMR_SPEC>;
#[doc = "Register `DMABMR` writer"]
pub type W = crate::W<DMABMR_SPEC>;
#[doc = "Field `SR` reader - Software reset"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Software reset"]
pub type SR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RTPR` reader - Rx Tx priority ratio"]
pub type RTPR_R = crate::FieldReader;
#[doc = "Field `RTPR` writer - Rx Tx priority ratio"]
pub type RTPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RDP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type USP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPM` reader - 4xPBL mode"]
pub type FPM_R = crate::BitReader;
#[doc = "Field `FPM` writer - 4xPBL mode"]
pub type FPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AAB_R = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AAB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMABMR_SPEC, 0> {
        SR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<DMABMR_SPEC, 1> {
        DA_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMABMR_SPEC, 2> {
        DSL_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<DMABMR_SPEC, 8> {
        PBL_W::new(self)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn rtpr(&mut self) -> RTPR_W<DMABMR_SPEC, 14> {
        RTPR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMABMR_SPEC, 16> {
        FB_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<DMABMR_SPEC, 17> {
        RDP_W::new(self)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<DMABMR_SPEC, 23> {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpm(&mut self) -> FPM_W<DMABMR_SPEC, 24> {
        FPM_W::new(self)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<DMABMR_SPEC, 25> {
        AAB_W::new(self)
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
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabmr::R`](R) reader structure"]
impl crate::Readable for DMABMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmabmr::W`](W) writer structure"]
impl crate::Writable for DMABMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMABMR to value 0x2101"]
impl crate::Resettable for DMABMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
