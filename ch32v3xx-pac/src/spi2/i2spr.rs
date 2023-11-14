#[doc = "Register `I2SPR` reader"]
pub type R = crate::R<I2SPR_SPEC>;
#[doc = "Register `I2SPR` writer"]
pub type W = crate::W<I2SPR_SPEC>;
#[doc = "Field `I2SDIV` reader - I2S Linear prescaler"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S Linear prescaler"]
pub type I2SDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type ODD_R = crate::BitReader;
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type ODD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MCKOE_R = crate::BitReader;
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MCKOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<I2SPR_SPEC, 0> {
        I2SDIV_W::new(self)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<I2SPR_SPEC, 8> {
        ODD_W::new(self)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<I2SPR_SPEC, 9> {
        MCKOE_W::new(self)
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
#[doc = "I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SPR_SPEC;
impl crate::RegisterSpec for I2SPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2spr::R`](R) reader structure"]
impl crate::Readable for I2SPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2spr::W`](W) writer structure"]
impl crate::Writable for I2SPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SPR to value 0x0a"]
impl crate::Resettable for I2SPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
