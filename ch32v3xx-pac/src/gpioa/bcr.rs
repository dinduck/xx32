#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCR_SPEC>;
#[doc = "Field `BR0` writer - Reset bit 0"]
pub type BR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR1` writer - Reset bit 1"]
pub type BR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR2` writer - Reset bit 1"]
pub type BR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR3` writer - Reset bit 3"]
pub type BR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR4` writer - Reset bit 4"]
pub type BR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR5` writer - Reset bit 5"]
pub type BR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR6` writer - Reset bit 6"]
pub type BR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR7` writer - Reset bit 7"]
pub type BR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR8` writer - Reset bit 8"]
pub type BR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR9` writer - Reset bit 9"]
pub type BR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR10` writer - Reset bit 10"]
pub type BR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR11` writer - Reset bit 11"]
pub type BR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR12` writer - Reset bit 12"]
pub type BR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR13` writer - Reset bit 13"]
pub type BR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR14` writer - Reset bit 14"]
pub type BR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR15` writer - Reset bit 15"]
pub type BR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Reset bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BCR_SPEC, 0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BCR_SPEC, 1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BCR_SPEC, 2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Reset bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BCR_SPEC, 3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Reset bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BCR_SPEC, 4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Reset bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BCR_SPEC, 5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Reset bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BCR_SPEC, 6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Reset bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BCR_SPEC, 7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Reset bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BCR_SPEC, 8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Reset bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BCR_SPEC, 9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Reset bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BCR_SPEC, 10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Reset bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BCR_SPEC, 11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Reset bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BCR_SPEC, 12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Reset bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BCR_SPEC, 13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Reset bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BCR_SPEC, 14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Reset bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BCR_SPEC, 15> {
        BR15_W::new(self)
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
#[doc = "Port bit reset register (GPIOn_BCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
