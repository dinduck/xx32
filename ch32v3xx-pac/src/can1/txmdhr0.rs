#[doc = "Register `TXMDHR0` reader"]
pub type R = crate::R<TXMDHR0_SPEC>;
#[doc = "Register `TXMDHR0` writer"]
pub type W = crate::W<TXMDHR0_SPEC>;
#[doc = "Field `DATA4` reader - Data byte 4"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Data byte 4"]
pub type DATA4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA5` reader - Data byte 5"]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Data byte 5"]
pub type DATA5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA6` reader - Data byte 6"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data byte 6"]
pub type DATA6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA7` reader - Data byte 7"]
pub type DATA7_R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Data byte 7"]
pub type DATA7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<TXMDHR0_SPEC, 0> {
        DATA4_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<TXMDHR0_SPEC, 8> {
        DATA5_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<TXMDHR0_SPEC, 16> {
        DATA6_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<TXMDHR0_SPEC, 24> {
        DATA7_W::new(self)
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
#[doc = "CAN mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdhr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdhr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMDHR0_SPEC;
impl crate::RegisterSpec for TXMDHR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmdhr0::R`](R) reader structure"]
impl crate::Readable for TXMDHR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmdhr0::W`](W) writer structure"]
impl crate::Writable for TXMDHR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMDHR0 to value 0"]
impl crate::Resettable for TXMDHR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
