#[doc = "Register `VTFADDRR1` reader"]
pub type R = crate::R<VTFADDRR1_SPEC>;
#[doc = "Register `VTFADDRR1` writer"]
pub type W = crate::W<VTFADDRR1_SPEC>;
#[doc = "Field `VTF1EN` reader - VTF1EN"]
pub type VTF1EN_R = crate::BitReader;
#[doc = "Field `VTF1EN` writer - VTF1EN"]
pub type VTF1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR1` reader - ADDR1"]
pub type ADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR1` writer - ADDR1"]
pub type ADDR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bit 0 - VTF1EN"]
    #[inline(always)]
    pub fn vtf1en(&self) -> VTF1EN_R {
        VTF1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - VTF1EN"]
    #[inline(always)]
    #[must_use]
    pub fn vtf1en(&mut self) -> VTF1EN_W<VTFADDRR1_SPEC, 0> {
        VTF1EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<VTFADDRR1_SPEC, 1> {
        ADDR1_W::new(self)
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
#[doc = "Interrupt 1 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTFADDRR1_SPEC;
impl crate::RegisterSpec for VTFADDRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtfaddrr1::R`](R) reader structure"]
impl crate::Readable for VTFADDRR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtfaddrr1::W`](W) writer structure"]
impl crate::Writable for VTFADDRR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTFADDRR1 to value 0"]
impl crate::Resettable for VTFADDRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
