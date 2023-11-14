#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTR_SPEC>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTR_SPEC>;
#[doc = "Field `USBHDRST` reader - USBHD reset"]
pub type USBHDRST_R = crate::BitReader;
#[doc = "Field `USBHDRST` writer - USBHD reset"]
pub type USBHDRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DVPRST` reader - DVP reset"]
pub type DVPRST_R = crate::BitReader;
#[doc = "Field `ETHMACRST` reader - Ethernet MAC reset"]
pub type ETHMACRST_R = crate::BitReader;
#[doc = "Field `ETHMACRST` writer - Ethernet MAC reset"]
pub type ETHMACRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 12 - USBHD reset"]
    #[inline(always)]
    pub fn usbhdrst(&self) -> USBHDRST_R {
        USBHDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DVP reset"]
    #[inline(always)]
    pub fn dvprst(&self) -> DVPRST_R {
        DVPRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet MAC reset"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBHD reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbhdrst(&mut self) -> USBHDRST_W<AHBRSTR_SPEC, 12> {
        USBHDRST_W::new(self)
    }
    #[doc = "Bit 14 - Ethernet MAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<AHBRSTR_SPEC, 14> {
        ETHMACRST_W::new(self)
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
#[doc = "AHB reset register (RCC_APHBRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
