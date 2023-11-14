#[doc = "Register `RDATAR_DR_ACT_DCG` reader"]
pub type R = crate::R<RDATAR_DR_ACT_DCG_SPEC>;
#[doc = "Register `RDATAR_DR_ACT_DCG` writer"]
pub type W = crate::W<RDATAR_DR_ACT_DCG_SPEC>;
#[doc = "Field `DATA0_7_TKACT_DCG` reader - Regular data_Touch key start and discharge time register"]
pub type DATA0_7_TKACT_DCG_R = crate::FieldReader;
#[doc = "Field `DATA0_7_TKACT_DCG` writer - Regular data_Touch key start and discharge time register"]
pub type DATA0_7_TKACT_DCG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA8_15` reader - Regular data"]
pub type DATA8_15_R = crate::FieldReader;
#[doc = "Field `DATA8_15` writer - Regular data"]
pub type DATA8_15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Regular data_Touch key start and discharge time register"]
    #[inline(always)]
    pub fn data0_7_tkact_dcg(&self) -> DATA0_7_TKACT_DCG_R {
        DATA0_7_TKACT_DCG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular data"]
    #[inline(always)]
    pub fn data8_15(&self) -> DATA8_15_R {
        DATA8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular data_Touch key start and discharge time register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_7_tkact_dcg(&mut self) -> DATA0_7_TKACT_DCG_W<RDATAR_DR_ACT_DCG_SPEC, 0> {
        DATA0_7_TKACT_DCG_W::new(self)
    }
    #[doc = "Bits 8:15 - Regular data"]
    #[inline(always)]
    #[must_use]
    pub fn data8_15(&mut self) -> DATA8_15_W<RDATAR_DR_ACT_DCG_SPEC, 8> {
        DATA8_15_W::new(self)
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
#[doc = "regular data register_start and discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar_dr_act_dcg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdatar_dr_act_dcg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATAR_DR_ACT_DCG_SPEC;
impl crate::RegisterSpec for RDATAR_DR_ACT_DCG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdatar_dr_act_dcg::R`](R) reader structure"]
impl crate::Readable for RDATAR_DR_ACT_DCG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdatar_dr_act_dcg::W`](W) writer structure"]
impl crate::Writable for RDATAR_DR_ACT_DCG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDATAR_DR_ACT_DCG to value 0"]
impl crate::Resettable for RDATAR_DR_ACT_DCG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
