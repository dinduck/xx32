#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CTLR_SPEC>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `LPDS` reader - Low Power Deep Sleep"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - Low Power Deep Sleep"]
pub type LPDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDDS` reader - Power Down Deep Sleep"]
pub type PDDS_R = crate::BitReader;
#[doc = "Field `PDDS` writer - Power Down Deep Sleep"]
pub type PDDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CWUF` reader - Clear Wake-up Flag"]
pub type CWUF_R = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear Wake-up Flag"]
pub type CWUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSBF` reader - Clear STANDBY Flag"]
pub type CSBF_R = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear STANDBY Flag"]
pub type CSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVDE` reader - Power Voltage Detector Enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power Voltage Detector Enable"]
pub type PVDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLS` reader - PVD Level Selection"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD Level Selection"]
pub type PLS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DBP` reader - Disable Backup Domain write protection"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable Backup Domain write protection"]
pub type DBP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `R2K_STYEN` reader - standby 2k ram enable"]
pub type R2K_STYEN_R = crate::BitReader;
#[doc = "Field `R2K_STYEN` writer - standby 2k ram enable"]
pub type R2K_STYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `R30K_STYEN` reader - standby 30k ram enable"]
pub type R30K_STYEN_R = crate::BitReader;
#[doc = "Field `R30K_STYEN` writer - standby 30k ram enable"]
pub type R30K_STYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `R2K_VBATEN` reader - VBAT 30k ram enable"]
pub type R2K_VBATEN_R = crate::BitReader;
#[doc = "Field `R2K_VBATEN` writer - VBAT 30k ram enable"]
pub type R2K_VBATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `R30K_VBATEN` reader - VBAT 30k ram enable"]
pub type R30K_VBATEN_R = crate::BitReader;
#[doc = "Field `R30K_VBATEN` writer - VBAT 30k ram enable"]
pub type R30K_VBATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAM_LVEN` reader - Ram LV Enable"]
pub type RAM_LVEN_R = crate::BitReader;
#[doc = "Field `RAM_LVEN` writer - Ram LV Enable"]
pub type RAM_LVEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - standby 2k ram enable"]
    #[inline(always)]
    pub fn r2k_styen(&self) -> R2K_STYEN_R {
        R2K_STYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - standby 30k ram enable"]
    #[inline(always)]
    pub fn r30k_styen(&self) -> R30K_STYEN_R {
        R30K_STYEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r2k_vbaten(&self) -> R2K_VBATEN_R {
        R2K_VBATEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r30k_vbaten(&self) -> R30K_VBATEN_R {
        R30K_VBATEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ram LV Enable"]
    #[inline(always)]
    pub fn ram_lven(&self) -> RAM_LVEN_R {
        RAM_LVEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<CTLR_SPEC, 0> {
        LPDS_W::new(self)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<CTLR_SPEC, 1> {
        PDDS_W::new(self)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<CTLR_SPEC, 2> {
        CWUF_W::new(self)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<CTLR_SPEC, 3> {
        CSBF_W::new(self)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CTLR_SPEC, 4> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CTLR_SPEC, 5> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CTLR_SPEC, 8> {
        DBP_W::new(self)
    }
    #[doc = "Bit 16 - standby 2k ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2k_styen(&mut self) -> R2K_STYEN_W<CTLR_SPEC, 16> {
        R2K_STYEN_W::new(self)
    }
    #[doc = "Bit 17 - standby 30k ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn r30k_styen(&mut self) -> R30K_STYEN_W<CTLR_SPEC, 17> {
        R30K_STYEN_W::new(self)
    }
    #[doc = "Bit 18 - VBAT 30k ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2k_vbaten(&mut self) -> R2K_VBATEN_W<CTLR_SPEC, 18> {
        R2K_VBATEN_W::new(self)
    }
    #[doc = "Bit 19 - VBAT 30k ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn r30k_vbaten(&mut self) -> R30K_VBATEN_W<CTLR_SPEC, 19> {
        R30K_VBATEN_W::new(self)
    }
    #[doc = "Bit 20 - Ram LV Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram_lven(&mut self) -> RAM_LVEN_W<CTLR_SPEC, 20> {
        RAM_LVEN_W::new(self)
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
#[doc = "Power control register (PWR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
