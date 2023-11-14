#[doc = "Register `AHBPCENR` reader"]
pub type R = crate::R<AHBPCENR_SPEC>;
#[doc = "Register `AHBPCENR` writer"]
pub type W = crate::W<AHBPCENR_SPEC>;
#[doc = "Field `DMA1EN` reader - DMA clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA clock enable"]
pub type DMA1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SRAMEN_R = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SRAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub type FLITFEN_R = crate::BitReader;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub type FLITFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FSMCEN` reader - FSMC clock enable"]
pub type FSMCEN_R = crate::BitReader;
#[doc = "Field `FSMCEN` writer - FSMC clock enable"]
pub type FSMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNG_EN` reader - TRNG clock enable"]
pub type TRNG_EN_R = crate::BitReader;
#[doc = "Field `TRNG_EN` writer - TRNG clock enable"]
pub type TRNG_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub type SDIOEN_R = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub type SDIOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHS_EN` reader - USBHS clock enable"]
pub type USBHS_EN_R = crate::BitReader;
#[doc = "Field `USBHS_EN` writer - USBHS clock enable"]
pub type USBHS_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTG_EN` reader - OTG clock enable"]
pub type OTG_EN_R = crate::BitReader;
#[doc = "Field `OTG_EN` writer - OTG clock enable"]
pub type OTG_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DVP_EN` reader - DVP clock enable"]
pub type DVP_EN_R = crate::BitReader;
#[doc = "Field `DVP_EN` writer - DVP clock enable"]
pub type DVP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETHMACEN` reader - Ethernet MAC clock enable"]
pub type ETHMACEN_R = crate::BitReader;
#[doc = "Field `ETHMACEN` writer - Ethernet MAC clock enable"]
pub type ETHMACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETHMACTXEN` reader - Ethernet MAC TX clock enable"]
pub type ETHMACTXEN_R = crate::BitReader;
#[doc = "Field `ETHMACTXEN` writer - Ethernet MAC TX clock enable"]
pub type ETHMACTXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETHMACRXEN` reader - Ethernet MAC RX clock enable"]
pub type ETHMACRXEN_R = crate::BitReader;
#[doc = "Field `ETHMACRXEN` writer - Ethernet MAC RX clock enable"]
pub type ETHMACRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRNG clock enable"]
    #[inline(always)]
    pub fn trng_en(&self) -> TRNG_EN_R {
        TRNG_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhs_en(&self) -> USBHS_EN_R {
        USBHS_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG clock enable"]
    #[inline(always)]
    pub fn otg_en(&self) -> OTG_EN_R {
        OTG_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp_en(&self) -> DVP_EN_R {
        DVP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBPCENR_SPEC, 0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBPCENR_SPEC, 1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<AHBPCENR_SPEC, 2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<AHBPCENR_SPEC, 4> {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBPCENR_SPEC, 6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<AHBPCENR_SPEC, 8> {
        FSMCEN_W::new(self)
    }
    #[doc = "Bit 9 - TRNG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn trng_en(&mut self) -> TRNG_EN_W<AHBPCENR_SPEC, 9> {
        TRNG_EN_W::new(self)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<AHBPCENR_SPEC, 10> {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 11 - USBHS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_en(&mut self) -> USBHS_EN_W<AHBPCENR_SPEC, 11> {
        USBHS_EN_W::new(self)
    }
    #[doc = "Bit 12 - OTG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_en(&mut self) -> OTG_EN_W<AHBPCENR_SPEC, 12> {
        OTG_EN_W::new(self)
    }
    #[doc = "Bit 13 - DVP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_en(&mut self) -> DVP_EN_W<AHBPCENR_SPEC, 13> {
        DVP_EN_W::new(self)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<AHBPCENR_SPEC, 14> {
        ETHMACEN_W::new(self)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W<AHBPCENR_SPEC, 15> {
        ETHMACTXEN_W::new(self)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W<AHBPCENR_SPEC, 16> {
        ETHMACRXEN_W::new(self)
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
#[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbpcenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbpcenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBPCENR_SPEC;
impl crate::RegisterSpec for AHBPCENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbpcenr::R`](R) reader structure"]
impl crate::Readable for AHBPCENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbpcenr::W`](W) writer structure"]
impl crate::Writable for AHBPCENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBPCENR to value 0x14"]
impl crate::Resettable for AHBPCENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
