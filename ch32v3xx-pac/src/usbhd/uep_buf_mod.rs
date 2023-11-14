#[doc = "Register `UEP_BUF_MOD` reader"]
pub type R = crate::R<UEP_BUF_MOD_SPEC>;
#[doc = "Register `UEP_BUF_MOD` writer"]
pub type W = crate::W<UEP_BUF_MOD_SPEC>;
#[doc = "Field `bUEP_BUF_MOD` reader - buffer mode of USB endpoint"]
pub type B_UEP_BUF_MOD_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_BUF_MOD` writer - buffer mode of USB endpoint"]
pub type B_UEP_BUF_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `bUEP_ISO_BUF_MOD` reader - buffer mode of USB endpoint"]
pub type B_UEP_ISO_BUF_MOD_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_ISO_BUF_MOD` writer - buffer mode of USB endpoint"]
pub type B_UEP_ISO_BUF_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_buf_mod(&self) -> B_UEP_BUF_MOD_R {
        B_UEP_BUF_MOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_iso_buf_mod(&self) -> B_UEP_ISO_BUF_MOD_R {
        B_UEP_ISO_BUF_MOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - buffer mode of USB endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_buf_mod(&mut self) -> B_UEP_BUF_MOD_W<UEP_BUF_MOD_SPEC, 0> {
        B_UEP_BUF_MOD_W::new(self)
    }
    #[doc = "Bits 16:31 - buffer mode of USB endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_iso_buf_mod(&mut self) -> B_UEP_ISO_BUF_MOD_W<UEP_BUF_MOD_SPEC, 16> {
        B_UEP_ISO_BUF_MOD_W::new(self)
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
#[doc = "USB endpoint buffer mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_buf_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_buf_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP_BUF_MOD_SPEC;
impl crate::RegisterSpec for UEP_BUF_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uep_buf_mod::R`](R) reader structure"]
impl crate::Readable for UEP_BUF_MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep_buf_mod::W`](W) writer structure"]
impl crate::Writable for UEP_BUF_MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP_BUF_MOD to value 0"]
impl crate::Resettable for UEP_BUF_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
