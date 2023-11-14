#[doc = "Register `L12BDHR1` reader"]
pub type R = crate::R<L12BDHR1_SPEC>;
#[doc = "Register `L12BDHR1` writer"]
pub type W = crate::W<L12BDHR1_SPEC>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data"]
pub type DACC1DHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<L12BDHR1_SPEC, 4> {
        DACC1DHR_W::new(self)
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
#[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l12bdhr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l12bdhr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L12BDHR1_SPEC;
impl crate::RegisterSpec for L12BDHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l12bdhr1::R`](R) reader structure"]
impl crate::Readable for L12BDHR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l12bdhr1::W`](W) writer structure"]
impl crate::Writable for L12BDHR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L12BDHR1 to value 0"]
impl crate::Resettable for L12BDHR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
