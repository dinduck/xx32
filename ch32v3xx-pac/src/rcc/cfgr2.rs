#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "Field `PREDIV1` reader - PREDIV1 division factor"]
pub type PREDIV1_R = crate::FieldReader;
#[doc = "Field `PREDIV1` writer - PREDIV1 division factor"]
pub type PREDIV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PREDIV2` reader - PREDIV2 division factor"]
pub type PREDIV2_R = crate::FieldReader;
#[doc = "Field `PREDIV2` writer - PREDIV2 division factor"]
pub type PREDIV2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL2MUL` reader - PLL2 Multiplication Factor"]
pub type PLL2MUL_R = crate::FieldReader;
#[doc = "Field `PLL2MUL` writer - PLL2 Multiplication Factor"]
pub type PLL2MUL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL3MUL` reader - PLL3 Multiplication Factor"]
pub type PLL3MUL_R = crate::FieldReader;
#[doc = "Field `PLL3MUL` writer - PLL3 Multiplication Factor"]
pub type PLL3MUL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PREDIV1SRC` reader - PREDIV1 entry clock source"]
pub type PREDIV1SRC_R = crate::BitReader;
#[doc = "Field `PREDIV1SRC` writer - PREDIV1 entry clock source"]
pub type PREDIV1SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S2SRC` reader - I2S2 clock source"]
pub type I2S2SRC_R = crate::BitReader;
#[doc = "Field `I2S2SRC` writer - I2S2 clock source"]
pub type I2S2SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S3SRC` reader - I2S3 clock source"]
pub type I2S3SRC_R = crate::BitReader;
#[doc = "Field `I2S3SRC` writer - I2S3 clock source"]
pub type I2S3SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNG_SRC` reader - TRNG clock source"]
pub type TRNG_SRC_R = crate::BitReader;
#[doc = "Field `TRNG_SRC` writer - TRNG clock source"]
pub type TRNG_SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1G_SRC` reader - ETH1G clock source"]
pub type ETH1G_SRC_R = crate::FieldReader;
#[doc = "Field `ETH1G_SRC` writer - ETH1G clock source"]
pub type ETH1G_SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ETH1G_125M_EN` reader - ETH1G _125M clock enable"]
pub type ETH1G_125M_EN_R = crate::BitReader;
#[doc = "Field `ETH1G_125M_EN` writer - ETH1G _125M clock enable"]
pub type ETH1G_125M_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHS_PREDIY` reader - USB HS PREDIV division factor"]
pub type USBHS_PREDIY_R = crate::FieldReader;
#[doc = "Field `USBHS_PREDIY` writer - USB HS PREDIV division factor"]
pub type USBHS_PREDIY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `USBHS_PLL_SRC` reader - USB HS Multiplication Factor clock source"]
pub type USBHS_PLL_SRC_R = crate::BitReader;
#[doc = "Field `USBHS_PLL_SRC` writer - USB HS Multiplication Factor clock source"]
pub type USBHS_PLL_SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHS_CKPEF_SEL` reader - USB HS Peference Clock source"]
pub type USBHS_CKPEF_SEL_R = crate::FieldReader;
#[doc = "Field `USBHS_CKPEF_SEL` writer - USB HS Peference Clock source"]
pub type USBHS_CKPEF_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USBHS_PLLALIVE` reader - USB HS Multiplication control"]
pub type USBHS_PLLALIVE_R = crate::BitReader;
#[doc = "Field `USBHS_PLLALIVE` writer - USB HS Multiplication control"]
pub type USBHS_PLLALIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHS_CLK_SRC` reader - USB HS clock source"]
pub type USBHS_CLK_SRC_R = crate::BitReader;
#[doc = "Field `USBHS_CLK_SRC` writer - USB HS clock source"]
pub type USBHS_CLK_SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    pub fn prediv1(&self) -> PREDIV1_R {
        PREDIV1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline(always)]
    pub fn prediv2(&self) -> PREDIV2_R {
        PREDIV2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline(always)]
    pub fn pll2mul(&self) -> PLL2MUL_R {
        PLL2MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline(always)]
    pub fn pll3mul(&self) -> PLL3MUL_R {
        PLL3MUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline(always)]
    pub fn prediv1src(&self) -> PREDIV1SRC_R {
        PREDIV1SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline(always)]
    pub fn i2s3src(&self) -> I2S3SRC_R {
        I2S3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRNG clock source"]
    #[inline(always)]
    pub fn trng_src(&self) -> TRNG_SRC_R {
        TRNG_SRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ETH1G clock source"]
    #[inline(always)]
    pub fn eth1g_src(&self) -> ETH1G_SRC_R {
        ETH1G_SRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - ETH1G _125M clock enable"]
    #[inline(always)]
    pub fn eth1g_125m_en(&self) -> ETH1G_125M_EN_R {
        ETH1G_125M_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - USB HS PREDIV division factor"]
    #[inline(always)]
    pub fn usbhs_prediy(&self) -> USBHS_PREDIY_R {
        USBHS_PREDIY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - USB HS Multiplication Factor clock source"]
    #[inline(always)]
    pub fn usbhs_pll_src(&self) -> USBHS_PLL_SRC_R {
        USBHS_PLL_SRC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - USB HS Peference Clock source"]
    #[inline(always)]
    pub fn usbhs_ckpef_sel(&self) -> USBHS_CKPEF_SEL_R {
        USBHS_CKPEF_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - USB HS Multiplication control"]
    #[inline(always)]
    pub fn usbhs_pllalive(&self) -> USBHS_PLLALIVE_R {
        USBHS_PLLALIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB HS clock source"]
    #[inline(always)]
    pub fn usbhs_clk_src(&self) -> USBHS_CLK_SRC_R {
        USBHS_CLK_SRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn prediv1(&mut self) -> PREDIV1_W<CFGR2_SPEC, 0> {
        PREDIV1_W::new(self)
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn prediv2(&mut self) -> PREDIV2_W<CFGR2_SPEC, 4> {
        PREDIV2_W::new(self)
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll2mul(&mut self) -> PLL2MUL_W<CFGR2_SPEC, 8> {
        PLL2MUL_W::new(self)
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll3mul(&mut self) -> PLL3MUL_W<CFGR2_SPEC, 12> {
        PLL3MUL_W::new(self)
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn prediv1src(&mut self) -> PREDIV1SRC_W<CFGR2_SPEC, 16> {
        PREDIV1SRC_W::new(self)
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<CFGR2_SPEC, 17> {
        I2S2SRC_W::new(self)
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline(always)]
    #[must_use]
    pub fn i2s3src(&mut self) -> I2S3SRC_W<CFGR2_SPEC, 18> {
        I2S3SRC_W::new(self)
    }
    #[doc = "Bit 19 - TRNG clock source"]
    #[inline(always)]
    #[must_use]
    pub fn trng_src(&mut self) -> TRNG_SRC_W<CFGR2_SPEC, 19> {
        TRNG_SRC_W::new(self)
    }
    #[doc = "Bits 20:21 - ETH1G clock source"]
    #[inline(always)]
    #[must_use]
    pub fn eth1g_src(&mut self) -> ETH1G_SRC_W<CFGR2_SPEC, 20> {
        ETH1G_SRC_W::new(self)
    }
    #[doc = "Bit 22 - ETH1G _125M clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth1g_125m_en(&mut self) -> ETH1G_125M_EN_W<CFGR2_SPEC, 22> {
        ETH1G_125M_EN_W::new(self)
    }
    #[doc = "Bits 24:26 - USB HS PREDIV division factor"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_prediy(&mut self) -> USBHS_PREDIY_W<CFGR2_SPEC, 24> {
        USBHS_PREDIY_W::new(self)
    }
    #[doc = "Bit 27 - USB HS Multiplication Factor clock source"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_pll_src(&mut self) -> USBHS_PLL_SRC_W<CFGR2_SPEC, 27> {
        USBHS_PLL_SRC_W::new(self)
    }
    #[doc = "Bits 28:29 - USB HS Peference Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_ckpef_sel(&mut self) -> USBHS_CKPEF_SEL_W<CFGR2_SPEC, 28> {
        USBHS_CKPEF_SEL_W::new(self)
    }
    #[doc = "Bit 30 - USB HS Multiplication control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_pllalive(&mut self) -> USBHS_PLLALIVE_W<CFGR2_SPEC, 30> {
        USBHS_PLLALIVE_W::new(self)
    }
    #[doc = "Bit 31 - USB HS clock source"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_clk_src(&mut self) -> USBHS_CLK_SRC_W<CFGR2_SPEC, 31> {
        USBHS_CLK_SRC_W::new(self)
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
#[doc = "Clock configuration register2 (RCC_CFGR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
