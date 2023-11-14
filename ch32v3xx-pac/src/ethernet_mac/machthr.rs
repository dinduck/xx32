#[doc = "Register `MACHTHR` reader"]
pub type R = crate::R<MACHTHR_SPEC>;
#[doc = "Register `MACHTHR` writer"]
pub type W = crate::W<MACHTHR_SPEC>;
#[doc = "Field `HTH` reader - Hash table high"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash table high"]
pub type HTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<MACHTHR_SPEC, 0> {
        HTH_W::new(self)
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
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTHR_SPEC;
impl crate::RegisterSpec for MACHTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machthr::R`](R) reader structure"]
impl crate::Readable for MACHTHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machthr::W`](W) writer structure"]
impl crate::Writable for MACHTHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHTHR to value 0"]
impl crate::Resettable for MACHTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
