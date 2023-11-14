#[doc = "Register `VTFADDRR2` reader"]
pub type R = crate::R<VTFADDRR2_SPEC>;
#[doc = "Register `VTFADDRR2` writer"]
pub type W = crate::W<VTFADDRR2_SPEC>;
#[doc = "Field `VTF2EN` reader - VTF2EN"]
pub type VTF2EN_R = crate::BitReader;
#[doc = "Field `VTF2EN` writer - VTF2EN"]
pub type VTF2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR2` reader - ADDR2"]
pub type ADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR2` writer - ADDR2"]
pub type ADDR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bit 0 - VTF2EN"]
    #[inline(always)]
    pub fn vtf2en(&self) -> VTF2EN_R {
        VTF2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - VTF2EN"]
    #[inline(always)]
    #[must_use]
    pub fn vtf2en(&mut self) -> VTF2EN_W<VTFADDRR2_SPEC, 0> {
        VTF2EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR2"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> ADDR2_W<VTFADDRR2_SPEC, 1> {
        ADDR2_W::new(self)
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
#[doc = "Interrupt 2 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTFADDRR2_SPEC;
impl crate::RegisterSpec for VTFADDRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtfaddrr2::R`](R) reader structure"]
impl crate::Readable for VTFADDRR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtfaddrr2::W`](W) writer structure"]
impl crate::Writable for VTFADDRR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTFADDRR2 to value 0"]
impl crate::Resettable for VTFADDRR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
