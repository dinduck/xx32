#[doc = "Register `CFGLR` reader"]
pub type R = crate::R<CFGLR_SPEC>;
#[doc = "Register `CFGLR` writer"]
pub type W = crate::W<CFGLR_SPEC>;
#[doc = "Field `MODE0` reader - Port n.0 mode bits"]
pub type MODE0_R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Port n.0 mode bits"]
pub type MODE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF0` reader - Port n.0 configuration bits"]
pub type CNF0_R = crate::FieldReader;
#[doc = "Field `CNF0` writer - Port n.0 configuration bits"]
pub type CNF0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE1` reader - Port n.1 mode bits"]
pub type MODE1_R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Port n.1 mode bits"]
pub type MODE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF1` reader - Port n.1 configuration bits"]
pub type CNF1_R = crate::FieldReader;
#[doc = "Field `CNF1` writer - Port n.1 configuration bits"]
pub type CNF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE2` reader - Port n.2 mode bits"]
pub type MODE2_R = crate::FieldReader;
#[doc = "Field `MODE2` writer - Port n.2 mode bits"]
pub type MODE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF2` reader - Port n.2 configuration bits"]
pub type CNF2_R = crate::FieldReader;
#[doc = "Field `CNF2` writer - Port n.2 configuration bits"]
pub type CNF2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE3` reader - Port n.3 mode bits"]
pub type MODE3_R = crate::FieldReader;
#[doc = "Field `MODE3` writer - Port n.3 mode bits"]
pub type MODE3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF3` reader - Port n.3 configuration bits"]
pub type CNF3_R = crate::FieldReader;
#[doc = "Field `CNF3` writer - Port n.3 configuration bits"]
pub type CNF3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE4` reader - Port n.4 mode bits"]
pub type MODE4_R = crate::FieldReader;
#[doc = "Field `MODE4` writer - Port n.4 mode bits"]
pub type MODE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF4` reader - Port n.4 configuration bits"]
pub type CNF4_R = crate::FieldReader;
#[doc = "Field `CNF4` writer - Port n.4 configuration bits"]
pub type CNF4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE5` reader - Port n.5 mode bits"]
pub type MODE5_R = crate::FieldReader;
#[doc = "Field `MODE5` writer - Port n.5 mode bits"]
pub type MODE5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF5` reader - Port n.5 configuration bits"]
pub type CNF5_R = crate::FieldReader;
#[doc = "Field `CNF5` writer - Port n.5 configuration bits"]
pub type CNF5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE6` reader - Port n.6 mode bits"]
pub type MODE6_R = crate::FieldReader;
#[doc = "Field `MODE6` writer - Port n.6 mode bits"]
pub type MODE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF6` reader - Port n.6 configuration bits"]
pub type CNF6_R = crate::FieldReader;
#[doc = "Field `CNF6` writer - Port n.6 configuration bits"]
pub type CNF6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE7` reader - Port n.7 mode bits"]
pub type MODE7_R = crate::FieldReader;
#[doc = "Field `MODE7` writer - Port n.7 mode bits"]
pub type MODE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CNF7` reader - Port n.7 configuration bits"]
pub type CNF7_R = crate::FieldReader;
#[doc = "Field `CNF7` writer - Port n.7 configuration bits"]
pub type CNF7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> CNF0_R {
        CNF0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> CNF1_R {
        CNF1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> CNF2_R {
        CNF2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> CNF3_R {
        CNF3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> CNF4_R {
        CNF4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> CNF5_R {
        CNF5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> CNF6_R {
        CNF6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> CNF7_R {
        CNF7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<CFGLR_SPEC, 0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf0(&mut self) -> CNF0_W<CFGLR_SPEC, 2> {
        CNF0_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<CFGLR_SPEC, 4> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf1(&mut self) -> CNF1_W<CFGLR_SPEC, 6> {
        CNF1_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<CFGLR_SPEC, 8> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf2(&mut self) -> CNF2_W<CFGLR_SPEC, 10> {
        CNF2_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<CFGLR_SPEC, 12> {
        MODE3_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf3(&mut self) -> CNF3_W<CFGLR_SPEC, 14> {
        CNF3_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<CFGLR_SPEC, 16> {
        MODE4_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf4(&mut self) -> CNF4_W<CFGLR_SPEC, 18> {
        CNF4_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<CFGLR_SPEC, 20> {
        MODE5_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf5(&mut self) -> CNF5_W<CFGLR_SPEC, 22> {
        CNF5_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<CFGLR_SPEC, 24> {
        MODE6_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf6(&mut self) -> CNF6_W<CFGLR_SPEC, 26> {
        CNF6_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<CFGLR_SPEC, 28> {
        MODE7_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf7(&mut self) -> CNF7_W<CFGLR_SPEC, 30> {
        CNF7_W::new(self)
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
#[doc = "Port configuration register low (GPIOn_CFGLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGLR_SPEC;
impl crate::RegisterSpec for CFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfglr::R`](R) reader structure"]
impl crate::Readable for CFGLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfglr::W`](W) writer structure"]
impl crate::Writable for CFGLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGLR to value 0x4444_4444"]
impl crate::Resettable for CFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
