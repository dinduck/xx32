#[doc = "Register `GPR` reader"]
pub type R = crate::R<GPR_SPEC>;
#[doc = "Register `GPR` writer"]
pub type W = crate::W<GPR_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `GT` reader - Guard time value"]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value"]
pub type GT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<GPR_SPEC, 0> {
        PSC_W::new(self)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<GPR_SPEC, 8> {
        GT_W::new(self)
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
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPR_SPEC;
impl crate::RegisterSpec for GPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr::R`](R) reader structure"]
impl crate::Readable for GPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpr::W`](W) writer structure"]
impl crate::Writable for GPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR to value 0"]
impl crate::Resettable for GPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
