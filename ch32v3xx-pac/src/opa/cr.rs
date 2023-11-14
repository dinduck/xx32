#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `EN1` reader - OPA Enable1"]
pub type EN1_R = crate::BitReader;
#[doc = "Field `EN1` writer - OPA Enable1"]
pub type EN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE1` reader - OPA MODE1"]
pub type MODE1_R = crate::BitReader;
#[doc = "Field `MODE1` writer - OPA MODE1"]
pub type MODE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSEL1` reader - OPA NSEL1"]
pub type NSEL1_R = crate::BitReader;
#[doc = "Field `NSEL1` writer - OPA NSEL1"]
pub type NSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEL1` reader - OPA PSEL1"]
pub type PSEL1_R = crate::BitReader;
#[doc = "Field `PSEL1` writer - OPA PSEL1"]
pub type PSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN2` reader - OPA Enable2"]
pub type EN2_R = crate::BitReader;
#[doc = "Field `EN2` writer - OPA Enable2"]
pub type EN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE2` reader - OPA MODE2"]
pub type MODE2_R = crate::BitReader;
#[doc = "Field `MODE2` writer - OPA MODE2"]
pub type MODE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSEL2` reader - OPA NSEL2"]
pub type NSEL2_R = crate::BitReader;
#[doc = "Field `NSEL2` writer - OPA NSEL2"]
pub type NSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEL2` reader - OPA PSEL2"]
pub type PSEL2_R = crate::BitReader;
#[doc = "Field `PSEL2` writer - OPA PSEL2"]
pub type PSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN3` reader - OPA Eable3"]
pub type EN3_R = crate::BitReader;
#[doc = "Field `EN3` writer - OPA Eable3"]
pub type EN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE3` reader - OPA MODE3"]
pub type MODE3_R = crate::BitReader;
#[doc = "Field `MODE3` writer - OPA MODE3"]
pub type MODE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSEL3` reader - OPA NSEL3"]
pub type NSEL3_R = crate::BitReader;
#[doc = "Field `NSEL3` writer - OPA NSEL3"]
pub type NSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEL3` reader - OPA PSEL3"]
pub type PSEL3_R = crate::BitReader;
#[doc = "Field `PSEL3` writer - OPA PSEL3"]
pub type PSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN4` reader - OPA Enable4"]
pub type EN4_R = crate::BitReader;
#[doc = "Field `EN4` writer - OPA Enable4"]
pub type EN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE4` reader - OPA MODE4"]
pub type MODE4_R = crate::BitReader;
#[doc = "Field `MODE4` writer - OPA MODE4"]
pub type MODE4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSEL4` reader - OPA NSEL4"]
pub type NSEL4_R = crate::BitReader;
#[doc = "Field `NSEL4` writer - OPA NSEL4"]
pub type NSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEL4` reader - OPA PSEL4"]
pub type PSEL4_R = crate::BitReader;
#[doc = "Field `PSEL4` writer - OPA PSEL4"]
pub type PSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - OPA Enable1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPA MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPA NSEL1"]
    #[inline(always)]
    pub fn nsel1(&self) -> NSEL1_R {
        NSEL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPA PSEL1"]
    #[inline(always)]
    pub fn psel1(&self) -> PSEL1_R {
        PSEL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OPA Enable2"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPA MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPA NSEL2"]
    #[inline(always)]
    pub fn nsel2(&self) -> NSEL2_R {
        NSEL2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPA PSEL2"]
    #[inline(always)]
    pub fn psel2(&self) -> PSEL2_R {
        PSEL2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPA Eable3"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OPA MODE3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OPA NSEL3"]
    #[inline(always)]
    pub fn nsel3(&self) -> NSEL3_R {
        NSEL3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OPA PSEL3"]
    #[inline(always)]
    pub fn psel3(&self) -> PSEL3_R {
        PSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPA Enable4"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPA MODE4"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPA NSEL4"]
    #[inline(always)]
    pub fn nsel4(&self) -> NSEL4_R {
        NSEL4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OPA PSEL4"]
    #[inline(always)]
    pub fn psel4(&self) -> PSEL4_R {
        PSEL4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA Enable1"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CR_SPEC, 0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 1 - OPA MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<CR_SPEC, 1> {
        MODE1_W::new(self)
    }
    #[doc = "Bit 2 - OPA NSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn nsel1(&mut self) -> NSEL1_W<CR_SPEC, 2> {
        NSEL1_W::new(self)
    }
    #[doc = "Bit 3 - OPA PSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn psel1(&mut self) -> PSEL1_W<CR_SPEC, 3> {
        PSEL1_W::new(self)
    }
    #[doc = "Bit 4 - OPA Enable2"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CR_SPEC, 4> {
        EN2_W::new(self)
    }
    #[doc = "Bit 5 - OPA MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<CR_SPEC, 5> {
        MODE2_W::new(self)
    }
    #[doc = "Bit 6 - OPA NSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn nsel2(&mut self) -> NSEL2_W<CR_SPEC, 6> {
        NSEL2_W::new(self)
    }
    #[doc = "Bit 7 - OPA PSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn psel2(&mut self) -> PSEL2_W<CR_SPEC, 7> {
        PSEL2_W::new(self)
    }
    #[doc = "Bit 8 - OPA Eable3"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN3_W<CR_SPEC, 8> {
        EN3_W::new(self)
    }
    #[doc = "Bit 9 - OPA MODE3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<CR_SPEC, 9> {
        MODE3_W::new(self)
    }
    #[doc = "Bit 10 - OPA NSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn nsel3(&mut self) -> NSEL3_W<CR_SPEC, 10> {
        NSEL3_W::new(self)
    }
    #[doc = "Bit 11 - OPA PSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn psel3(&mut self) -> PSEL3_W<CR_SPEC, 11> {
        PSEL3_W::new(self)
    }
    #[doc = "Bit 12 - OPA Enable4"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN4_W<CR_SPEC, 12> {
        EN4_W::new(self)
    }
    #[doc = "Bit 13 - OPA MODE4"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<CR_SPEC, 13> {
        MODE4_W::new(self)
    }
    #[doc = "Bit 14 - OPA NSEL4"]
    #[inline(always)]
    #[must_use]
    pub fn nsel4(&mut self) -> NSEL4_W<CR_SPEC, 14> {
        NSEL4_W::new(self)
    }
    #[doc = "Bit 15 - OPA PSEL4"]
    #[inline(always)]
    #[must_use]
    pub fn psel4(&mut self) -> PSEL4_W<CR_SPEC, 15> {
        PSEL4_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
