#[doc = "Register `MMCRIMR` reader"]
pub type R = crate::R<MMCRIMR_SPEC>;
#[doc = "Register `MMCRIMR` writer"]
pub type W = crate::W<MMCRIMR_SPEC>;
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RFCEM_R = crate::BitReader;
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RFCEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RFAEM_R = crate::BitReader;
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RFAEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RGUFM` reader - Received good unicast frames mask"]
pub type RGUFM_R = crate::BitReader;
#[doc = "Field `RGUFM` writer - Received good unicast frames mask"]
pub type RGUFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfcem(&mut self) -> RFCEM_W<MMCRIMR_SPEC, 5> {
        RFCEM_W::new(self)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfaem(&mut self) -> RFAEM_W<MMCRIMR_SPEC, 6> {
        RFAEM_W::new(self)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgufm(&mut self) -> RGUFM_W<MMCRIMR_SPEC, 17> {
        RGUFM_W::new(self)
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
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIMR_SPEC;
impl crate::RegisterSpec for MMCRIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrimr::R`](R) reader structure"]
impl crate::Readable for MMCRIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcrimr::W`](W) writer structure"]
impl crate::Writable for MMCRIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MMCRIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
