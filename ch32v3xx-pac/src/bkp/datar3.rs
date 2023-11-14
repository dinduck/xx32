#[doc = "Register `DATAR3` reader"]
pub type R = crate::R<DATAR3_SPEC>;
#[doc = "Register `DATAR3` writer"]
pub type W = crate::W<DATAR3_SPEC>;
#[doc = "Field `D3` reader - Backup data"]
pub type D3_R = crate::FieldReader<u16>;
#[doc = "Field `D3` writer - Backup data"]
pub type D3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d3(&mut self) -> D3_W<DATAR3_SPEC, 0> {
        D3_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR3_SPEC;
impl crate::RegisterSpec for DATAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar3::R`](R) reader structure"]
impl crate::Readable for DATAR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar3::W`](W) writer structure"]
impl crate::Writable for DATAR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR3 to value 0"]
impl crate::Resettable for DATAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
