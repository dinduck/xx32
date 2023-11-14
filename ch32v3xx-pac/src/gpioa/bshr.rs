#[doc = "Register `BSHR` writer"]
pub type W = crate::W<BSHR_SPEC>;
#[doc = "Field `BS0` writer - Set bit 0"]
pub type BS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS1` writer - Set bit 1"]
pub type BS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS2` writer - Set bit 1"]
pub type BS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS3` writer - Set bit 3"]
pub type BS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS4` writer - Set bit 4"]
pub type BS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS5` writer - Set bit 5"]
pub type BS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS6` writer - Set bit 6"]
pub type BS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS7` writer - Set bit 7"]
pub type BS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS8` writer - Set bit 8"]
pub type BS8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS9` writer - Set bit 9"]
pub type BS9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS10` writer - Set bit 10"]
pub type BS10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS11` writer - Set bit 11"]
pub type BS11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS12` writer - Set bit 12"]
pub type BS12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS13` writer - Set bit 13"]
pub type BS13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS14` writer - Set bit 14"]
pub type BS14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS15` writer - Set bit 15"]
pub type BS15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR0` writer - Reset bit 0"]
pub type BR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR1` writer - Reset bit 1"]
pub type BR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR2` writer - Reset bit 2"]
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
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<BSHR_SPEC, 0> {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BSHR_SPEC, 1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 2 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BSHR_SPEC, 2> {
        BS2_W::new(self)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSHR_SPEC, 3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<BSHR_SPEC, 4> {
        BS4_W::new(self)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<BSHR_SPEC, 5> {
        BS5_W::new(self)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<BSHR_SPEC, 6> {
        BS6_W::new(self)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<BSHR_SPEC, 7> {
        BS7_W::new(self)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<BSHR_SPEC, 8> {
        BS8_W::new(self)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<BSHR_SPEC, 9> {
        BS9_W::new(self)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<BSHR_SPEC, 10> {
        BS10_W::new(self)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<BSHR_SPEC, 11> {
        BS11_W::new(self)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<BSHR_SPEC, 12> {
        BS12_W::new(self)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<BSHR_SPEC, 13> {
        BS13_W::new(self)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<BSHR_SPEC, 14> {
        BS14_W::new(self)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<BSHR_SPEC, 15> {
        BS15_W::new(self)
    }
    #[doc = "Bit 16 - Reset bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BSHR_SPEC, 16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BSHR_SPEC, 17> {
        BR1_W::new(self)
    }
    #[doc = "Bit 18 - Reset bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BSHR_SPEC, 18> {
        BR2_W::new(self)
    }
    #[doc = "Bit 19 - Reset bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSHR_SPEC, 19> {
        BR3_W::new(self)
    }
    #[doc = "Bit 20 - Reset bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BSHR_SPEC, 20> {
        BR4_W::new(self)
    }
    #[doc = "Bit 21 - Reset bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BSHR_SPEC, 21> {
        BR5_W::new(self)
    }
    #[doc = "Bit 22 - Reset bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BSHR_SPEC, 22> {
        BR6_W::new(self)
    }
    #[doc = "Bit 23 - Reset bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BSHR_SPEC, 23> {
        BR7_W::new(self)
    }
    #[doc = "Bit 24 - Reset bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BSHR_SPEC, 24> {
        BR8_W::new(self)
    }
    #[doc = "Bit 25 - Reset bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BSHR_SPEC, 25> {
        BR9_W::new(self)
    }
    #[doc = "Bit 26 - Reset bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BSHR_SPEC, 26> {
        BR10_W::new(self)
    }
    #[doc = "Bit 27 - Reset bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BSHR_SPEC, 27> {
        BR11_W::new(self)
    }
    #[doc = "Bit 28 - Reset bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BSHR_SPEC, 28> {
        BR12_W::new(self)
    }
    #[doc = "Bit 29 - Reset bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BSHR_SPEC, 29> {
        BR13_W::new(self)
    }
    #[doc = "Bit 30 - Reset bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BSHR_SPEC, 30> {
        BR14_W::new(self)
    }
    #[doc = "Bit 31 - Reset bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BSHR_SPEC, 31> {
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
#[doc = "Port bit set/reset register (GPIOn_BSHR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bshr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSHR_SPEC;
impl crate::RegisterSpec for BSHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bshr::W`](W) writer structure"]
impl crate::Writable for BSHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSHR to value 0"]
impl crate::Resettable for BSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
