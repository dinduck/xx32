#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<OADDR1_SPEC>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<OADDR1_SPEC>;
#[doc = "Field `ADD0` reader - Interface address"]
pub type ADD0_R = crate::BitReader;
#[doc = "Field `ADD0` writer - Interface address"]
pub type ADD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD7_1` reader - Interface address"]
pub type ADD7_1_R = crate::FieldReader;
#[doc = "Field `ADD7_1` writer - Interface address"]
pub type ADD7_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ADD9_8` reader - Interface address"]
pub type ADD9_8_R = crate::FieldReader;
#[doc = "Field `ADD9_8` writer - Interface address"]
pub type ADD9_8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MUST1` reader - Must be 1"]
pub type MUST1_R = crate::BitReader;
#[doc = "Field `MUST1` writer - Must be 1"]
pub type MUST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type ADDMODE_R = crate::BitReader;
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type ADDMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7_1(&self) -> ADD7_1_R {
        ADD7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add9_8(&self) -> ADD9_8_R {
        ADD9_8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 14 - Must be 1"]
    #[inline(always)]
    pub fn must1(&self) -> MUST1_R {
        MUST1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> ADD0_W<OADDR1_SPEC, 0> {
        ADD0_W::new(self)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add7_1(&mut self) -> ADD7_1_W<OADDR1_SPEC, 1> {
        ADD7_1_W::new(self)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add9_8(&mut self) -> ADD9_8_W<OADDR1_SPEC, 8> {
        ADD9_8_W::new(self)
    }
    #[doc = "Bit 14 - Must be 1"]
    #[inline(always)]
    #[must_use]
    pub fn must1(&mut self) -> MUST1_W<OADDR1_SPEC, 14> {
        MUST1_W::new(self)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> ADDMODE_W<OADDR1_SPEC, 15> {
        ADDMODE_W::new(self)
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
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
