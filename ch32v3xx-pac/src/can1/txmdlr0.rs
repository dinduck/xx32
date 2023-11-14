#[doc = "Register `TXMDLR0` reader"]
pub type R = crate::R<TXMDLR0_SPEC>;
#[doc = "Register `TXMDLR0` writer"]
pub type W = crate::W<TXMDLR0_SPEC>;
#[doc = "Field `DATA0` reader - Data byte 0"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA0` writer - Data byte 0"]
pub type DATA0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA1` reader - Data byte 1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Data byte 1"]
pub type DATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA2` reader - Data byte 2"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Data byte 2"]
pub type DATA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA3` reader - Data byte 3"]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Data byte 3"]
pub type DATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<TXMDLR0_SPEC, 0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<TXMDLR0_SPEC, 8> {
        DATA1_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<TXMDLR0_SPEC, 16> {
        DATA2_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<TXMDLR0_SPEC, 24> {
        DATA3_W::new(self)
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
#[doc = "CAN mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMDLR0_SPEC;
impl crate::RegisterSpec for TXMDLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmdlr0::R`](R) reader structure"]
impl crate::Readable for TXMDLR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmdlr0::W`](W) writer structure"]
impl crate::Writable for TXMDLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMDLR0 to value 0"]
impl crate::Resettable for TXMDLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
