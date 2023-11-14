#[doc = "Register `PTPTSLUR` reader"]
pub type R = crate::R<PTPTSLUR_SPEC>;
#[doc = "Register `PTPTSLUR` writer"]
pub type W = crate::W<PTPTSLUR_SPEC>;
#[doc = "Field `TSUSS` reader - Time stamp update subseconds"]
pub type TSUSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSUSS` writer - Time stamp update subseconds"]
pub type TSUSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `TSUPNS` reader - Time stamp update positive or negative sign"]
pub type TSUPNS_R = crate::BitReader;
#[doc = "Field `TSUPNS` writer - Time stamp update positive or negative sign"]
pub type TSUPNS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tsuss(&mut self) -> TSUSS_W<PTPTSLUR_SPEC, 0> {
        TSUSS_W::new(self)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    #[must_use]
    pub fn tsupns(&mut self) -> TSUPNS_W<PTPTSLUR_SPEC, 31> {
        TSUPNS_W::new(self)
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
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSLUR_SPEC;
impl crate::RegisterSpec for PTPTSLUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslur::R`](R) reader structure"]
impl crate::Readable for PTPTSLUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptslur::W`](W) writer structure"]
impl crate::Writable for PTPTSLUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSLUR to value 0"]
impl crate::Resettable for PTPTSLUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
