#[doc = "Register `VTFADDRR3` reader"]
pub type R = crate::R<VTFADDRR3_SPEC>;
#[doc = "Register `VTFADDRR3` writer"]
pub type W = crate::W<VTFADDRR3_SPEC>;
#[doc = "Field `VTF3EN` reader - VTF3EN"]
pub type VTF3EN_R = crate::BitReader;
#[doc = "Field `VTF3EN` writer - VTF3EN"]
pub type VTF3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR3` reader - ADDR3"]
pub type ADDR3_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR3` writer - ADDR3"]
pub type ADDR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bit 0 - VTF3EN"]
    #[inline(always)]
    pub fn vtf3en(&self) -> VTF3EN_R {
        VTF3EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR3"]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - VTF3EN"]
    #[inline(always)]
    #[must_use]
    pub fn vtf3en(&mut self) -> VTF3EN_W<VTFADDRR3_SPEC, 0> {
        VTF3EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR3"]
    #[inline(always)]
    #[must_use]
    pub fn addr3(&mut self) -> ADDR3_W<VTFADDRR3_SPEC, 1> {
        ADDR3_W::new(self)
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
#[doc = "Interrupt 3 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTFADDRR3_SPEC;
impl crate::RegisterSpec for VTFADDRR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtfaddrr3::R`](R) reader structure"]
impl crate::Readable for VTFADDRR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtfaddrr3::W`](W) writer structure"]
impl crate::Writable for VTFADDRR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTFADDRR3 to value 0"]
impl crate::Resettable for VTFADDRR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
