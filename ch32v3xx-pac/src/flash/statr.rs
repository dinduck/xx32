#[doc = "Register `STATR` reader"]
pub type R = crate::R<STATR_SPEC>;
#[doc = "Register `STATR` writer"]
pub type W = crate::W<STATR_SPEC>;
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader;
#[doc = "Field `WR_BSY` reader - Quick page programming"]
pub type WR_BSY_R = crate::BitReader;
#[doc = "Field `WRPRTERR` reader - Write protection error"]
pub type WRPRTERR_R = crate::BitReader;
#[doc = "Field `WRPRTERR` writer - Write protection error"]
pub type WRPRTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENHANCE_MOD_STA` reader - Enhance mode start"]
pub type ENHANCE_MOD_STA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Quick page programming"]
    #[inline(always)]
    pub fn wr_bsy(&self) -> WR_BSY_R {
        WR_BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Enhance mode start"]
    #[inline(always)]
    pub fn enhance_mod_sta(&self) -> ENHANCE_MOD_STA_R {
        ENHANCE_MOD_STA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    #[must_use]
    pub fn wrprterr(&mut self) -> WRPRTERR_W<STATR_SPEC, 4> {
        WRPRTERR_W::new(self)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<STATR_SPEC, 5> {
        EOP_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr::R`](R) reader structure"]
impl crate::Readable for STATR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statr::W`](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATR to value 0"]
impl crate::Resettable for STATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
