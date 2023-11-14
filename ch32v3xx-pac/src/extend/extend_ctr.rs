#[doc = "Register `EXTEND_CTR` reader"]
pub type R = crate::R<EXTEND_CTR_SPEC>;
#[doc = "Register `EXTEND_CTR` writer"]
pub type W = crate::W<EXTEND_CTR_SPEC>;
#[doc = "Field `USBDLS` reader - USBD Lowspeed Enable"]
pub type USBDLS_R = crate::BitReader;
#[doc = "Field `USBDLS` writer - USBD Lowspeed Enable"]
pub type USBDLS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBDPU` reader - USBD pullup Enable"]
pub type USBDPU_R = crate::BitReader;
#[doc = "Field `USBDPU` writer - USBD pullup Enable"]
pub type USBDPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH_10M_EN` reader - ETH 10M Enable"]
pub type ETH_10M_EN_R = crate::BitReader;
#[doc = "Field `ETH_10M_EN` writer - ETH 10M Enable"]
pub type ETH_10M_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH_RGMII_EN` reader - ETH RGMII Enable"]
pub type ETH_RGMII_EN_R = crate::BitReader;
#[doc = "Field `ETH_RGMII_EN` writer - ETH RGMII Enable"]
pub type ETH_RGMII_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLL_HSI_PRE` reader - Whether HSI is divided"]
pub type PLL_HSI_PRE_R = crate::BitReader;
#[doc = "Field `PLL_HSI_PRE` writer - Whether HSI is divided"]
pub type PLL_HSI_PRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKUP_EN` reader - LOCKUP_Eable"]
pub type LOCKUP_EN_R = crate::BitReader;
#[doc = "Field `LOCKUP_EN` writer - LOCKUP_Eable"]
pub type LOCKUP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKUP_RSTF` reader - LOCKUP RESET"]
pub type LOCKUP_RSTF_R = crate::BitReader;
#[doc = "Field `LOCKUP_RSTF` writer - LOCKUP RESET"]
pub type LOCKUP_RSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULLDO_TRIM` reader - ULLDO_TRIM"]
pub type ULLDO_TRIM_R = crate::FieldReader;
#[doc = "Field `ULLDO_TRIM` writer - ULLDO_TRIM"]
pub type ULLDO_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LDO_TRIM` reader - LDO_TRIM"]
pub type LDO_TRIM_R = crate::FieldReader;
#[doc = "Field `LDO_TRIM` writer - LDO_TRIM"]
pub type LDO_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HSE_KEEP_LP` reader - HSE_KEEP_LP"]
pub type HSE_KEEP_LP_R = crate::BitReader;
#[doc = "Field `HSE_KEEP_LP` writer - HSE_KEEP_LP"]
pub type HSE_KEEP_LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USBD Lowspeed Enable"]
    #[inline(always)]
    pub fn usbdls(&self) -> USBDLS_R {
        USBDLS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBD pullup Enable"]
    #[inline(always)]
    pub fn usbdpu(&self) -> USBDPU_R {
        USBDPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH 10M Enable"]
    #[inline(always)]
    pub fn eth_10m_en(&self) -> ETH_10M_EN_R {
        ETH_10M_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ETH RGMII Enable"]
    #[inline(always)]
    pub fn eth_rgmii_en(&self) -> ETH_RGMII_EN_R {
        ETH_RGMII_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Whether HSI is divided"]
    #[inline(always)]
    pub fn pll_hsi_pre(&self) -> PLL_HSI_PRE_R {
        PLL_HSI_PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - LOCKUP_Eable"]
    #[inline(always)]
    pub fn lockup_en(&self) -> LOCKUP_EN_R {
        LOCKUP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCKUP RESET"]
    #[inline(always)]
    pub fn lockup_rstf(&self) -> LOCKUP_RSTF_R {
        LOCKUP_RSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ULLDO_TRIM"]
    #[inline(always)]
    pub fn ulldo_trim(&self) -> ULLDO_TRIM_R {
        ULLDO_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LDO_TRIM"]
    #[inline(always)]
    pub fn ldo_trim(&self) -> LDO_TRIM_R {
        LDO_TRIM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - HSE_KEEP_LP"]
    #[inline(always)]
    pub fn hse_keep_lp(&self) -> HSE_KEEP_LP_R {
        HSE_KEEP_LP_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBD Lowspeed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbdls(&mut self) -> USBDLS_W<EXTEND_CTR_SPEC, 0> {
        USBDLS_W::new(self)
    }
    #[doc = "Bit 1 - USBD pullup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbdpu(&mut self) -> USBDPU_W<EXTEND_CTR_SPEC, 1> {
        USBDPU_W::new(self)
    }
    #[doc = "Bit 2 - ETH 10M Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth_10m_en(&mut self) -> ETH_10M_EN_W<EXTEND_CTR_SPEC, 2> {
        ETH_10M_EN_W::new(self)
    }
    #[doc = "Bit 3 - ETH RGMII Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth_rgmii_en(&mut self) -> ETH_RGMII_EN_W<EXTEND_CTR_SPEC, 3> {
        ETH_RGMII_EN_W::new(self)
    }
    #[doc = "Bit 4 - Whether HSI is divided"]
    #[inline(always)]
    #[must_use]
    pub fn pll_hsi_pre(&mut self) -> PLL_HSI_PRE_W<EXTEND_CTR_SPEC, 4> {
        PLL_HSI_PRE_W::new(self)
    }
    #[doc = "Bit 6 - LOCKUP_Eable"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_en(&mut self) -> LOCKUP_EN_W<EXTEND_CTR_SPEC, 6> {
        LOCKUP_EN_W::new(self)
    }
    #[doc = "Bit 7 - LOCKUP RESET"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_rstf(&mut self) -> LOCKUP_RSTF_W<EXTEND_CTR_SPEC, 7> {
        LOCKUP_RSTF_W::new(self)
    }
    #[doc = "Bits 8:9 - ULLDO_TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn ulldo_trim(&mut self) -> ULLDO_TRIM_W<EXTEND_CTR_SPEC, 8> {
        ULLDO_TRIM_W::new(self)
    }
    #[doc = "Bits 10:11 - LDO_TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_trim(&mut self) -> LDO_TRIM_W<EXTEND_CTR_SPEC, 10> {
        LDO_TRIM_W::new(self)
    }
    #[doc = "Bit 12 - HSE_KEEP_LP"]
    #[inline(always)]
    #[must_use]
    pub fn hse_keep_lp(&mut self) -> HSE_KEEP_LP_W<EXTEND_CTR_SPEC, 12> {
        HSE_KEEP_LP_W::new(self)
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
#[doc = "EXTEND register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extend_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extend_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTEND_CTR_SPEC;
impl crate::RegisterSpec for EXTEND_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extend_ctr::R`](R) reader structure"]
impl crate::Readable for EXTEND_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extend_ctr::W`](W) writer structure"]
impl crate::Writable for EXTEND_CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTEND_CTR to value 0x0a00"]
impl crate::Resettable for EXTEND_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00;
}
