#[doc = "Register `TPCSR` reader"]
pub type R = crate::R<TPCSR_SPEC>;
#[doc = "Register `TPCSR` writer"]
pub type W = crate::W<TPCSR_SPEC>;
#[doc = "Field `CTE` writer - Clear Tamper event"]
pub type CTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTI` writer - Clear Tamper Interrupt"]
pub type CTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIE` reader - Tamper Pin interrupt enable"]
pub type TPIE_R = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper Pin interrupt enable"]
pub type TPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEF` reader - Tamper Event Flag"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TIF` reader - Tamper Interrupt Flag"]
pub type TIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper Event Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Tamper event"]
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CTE_W<TPCSR_SPEC, 0> {
        CTE_W::new(self)
    }
    #[doc = "Bit 1 - Clear Tamper Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cti(&mut self) -> CTI_W<TPCSR_SPEC, 1> {
        CTI_W::new(self)
    }
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<TPCSR_SPEC, 2> {
        TPIE_W::new(self)
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
#[doc = "BKP_TPCSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCSR_SPEC;
impl crate::RegisterSpec for TPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpcsr::R`](R) reader structure"]
impl crate::Readable for TPCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpcsr::W`](W) writer structure"]
impl crate::Writable for TPCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCSR to value 0"]
impl crate::Resettable for TPCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
