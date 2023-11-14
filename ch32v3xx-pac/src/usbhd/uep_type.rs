#[doc = "Register `UEP_TYPE` reader"]
pub type R = crate::R<UEP_TYPE_SPEC>;
#[doc = "Register `UEP_TYPE` writer"]
pub type W = crate::W<UEP_TYPE_SPEC>;
#[doc = "Field `bUEP_T_TYPE` reader - endpoint TX type"]
pub type B_UEP_T_TYPE_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_T_TYPE` writer - endpoint TX type"]
pub type B_UEP_T_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `bUEP_R_TYPE` reader - endpoint RX type"]
pub type B_UEP_R_TYPE_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_R_TYPE` writer - endpoint RX type"]
pub type B_UEP_R_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
impl R {
    #[doc = "Bits 1:15 - endpoint TX type"]
    #[inline(always)]
    pub fn b_uep_t_type(&self) -> B_UEP_T_TYPE_R {
        B_UEP_T_TYPE_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:31 - endpoint RX type"]
    #[inline(always)]
    pub fn b_uep_r_type(&self) -> B_UEP_R_TYPE_R {
        B_UEP_R_TYPE_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - endpoint TX type"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_t_type(&mut self) -> B_UEP_T_TYPE_W<UEP_TYPE_SPEC, 1> {
        B_UEP_T_TYPE_W::new(self)
    }
    #[doc = "Bits 17:31 - endpoint RX type"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_r_type(&mut self) -> B_UEP_R_TYPE_W<UEP_TYPE_SPEC, 17> {
        B_UEP_R_TYPE_W::new(self)
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
#[doc = "USB endpoint type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP_TYPE_SPEC;
impl crate::RegisterSpec for UEP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uep_type::R`](R) reader structure"]
impl crate::Readable for UEP_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep_type::W`](W) writer structure"]
impl crate::Writable for UEP_TYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP_TYPE to value 0"]
impl crate::Resettable for UEP_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
