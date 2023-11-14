#[doc = "Register `VTFADDRR0` reader"]
pub type R = crate::R<VTFADDRR0_SPEC>;
#[doc = "Register `VTFADDRR0` writer"]
pub type W = crate::W<VTFADDRR0_SPEC>;
#[doc = "Field `VTF0EN` reader - VTF0EN"]
pub type VTF0EN_R = crate::BitReader;
#[doc = "Field `VTF0EN` writer - VTF0EN"]
pub type VTF0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR0` reader - ADDR0"]
pub type ADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR0` writer - ADDR0"]
pub type ADDR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bit 0 - VTF0EN"]
    #[inline(always)]
    pub fn vtf0en(&self) -> VTF0EN_R {
        VTF0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - VTF0EN"]
    #[inline(always)]
    #[must_use]
    pub fn vtf0en(&mut self) -> VTF0EN_W<VTFADDRR0_SPEC, 0> {
        VTF0EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR0"]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> ADDR0_W<VTFADDRR0_SPEC, 1> {
        ADDR0_W::new(self)
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
#[doc = "Interrupt 0 address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfaddrr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfaddrr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTFADDRR0_SPEC;
impl crate::RegisterSpec for VTFADDRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtfaddrr0::R`](R) reader structure"]
impl crate::Readable for VTFADDRR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtfaddrr0::W`](W) writer structure"]
impl crate::Writable for VTFADDRR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTFADDRR0 to value 0"]
impl crate::Resettable for VTFADDRR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
