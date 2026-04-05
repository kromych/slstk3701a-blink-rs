#[doc = "Register `EXTILEVEL` reader"]
pub type R = crate::R<ExtilevelSpec>;
#[doc = "Register `EXTILEVEL` writer"]
pub type W = crate::W<ExtilevelSpec>;
#[doc = "Field `EM4WU0` reader - EM4 Wake Up Level for EM4WU0 Pin"]
pub type Em4wu0R = crate::BitReader;
#[doc = "Field `EM4WU0` writer - EM4 Wake Up Level for EM4WU0 Pin"]
pub type Em4wu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU1` reader - EM4 Wake Up Level for EM4WU1 Pin"]
pub type Em4wu1R = crate::BitReader;
#[doc = "Field `EM4WU1` writer - EM4 Wake Up Level for EM4WU1 Pin"]
pub type Em4wu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU2` reader - EM4 Wake Up Level for EM4WU2 Pin"]
pub type Em4wu2R = crate::BitReader;
#[doc = "Field `EM4WU2` writer - EM4 Wake Up Level for EM4WU2 Pin"]
pub type Em4wu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU3` reader - EM4 Wake Up Level for EM4WU3 Pin"]
pub type Em4wu3R = crate::BitReader;
#[doc = "Field `EM4WU3` writer - EM4 Wake Up Level for EM4WU3 Pin"]
pub type Em4wu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU4` reader - EM4 Wake Up Level for EM4WU4 Pin"]
pub type Em4wu4R = crate::BitReader;
#[doc = "Field `EM4WU4` writer - EM4 Wake Up Level for EM4WU4 Pin"]
pub type Em4wu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU5` reader - EM4 Wake Up Level for EM4WU5 Pin"]
pub type Em4wu5R = crate::BitReader;
#[doc = "Field `EM4WU5` writer - EM4 Wake Up Level for EM4WU5 Pin"]
pub type Em4wu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU6` reader - EM4 Wake Up Level for EM4WU6 Pin"]
pub type Em4wu6R = crate::BitReader;
#[doc = "Field `EM4WU6` writer - EM4 Wake Up Level for EM4WU6 Pin"]
pub type Em4wu6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU7` reader - EM4 Wake Up Level for EM4WU7 Pin"]
pub type Em4wu7R = crate::BitReader;
#[doc = "Field `EM4WU7` writer - EM4 Wake Up Level for EM4WU7 Pin"]
pub type Em4wu7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU8` reader - EM4 Wake Up Level for EM4WU8 Pin"]
pub type Em4wu8R = crate::BitReader;
#[doc = "Field `EM4WU8` writer - EM4 Wake Up Level for EM4WU8 Pin"]
pub type Em4wu8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU9` reader - EM4 Wake Up Level for EM4WU9 Pin"]
pub type Em4wu9R = crate::BitReader;
#[doc = "Field `EM4WU9` writer - EM4 Wake Up Level for EM4WU9 Pin"]
pub type Em4wu9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&self) -> Em4wu0R {
        Em4wu0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&self) -> Em4wu1R {
        Em4wu1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    pub fn em4wu2(&self) -> Em4wu2R {
        Em4wu2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    pub fn em4wu3(&self) -> Em4wu3R {
        Em4wu3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&self) -> Em4wu4R {
        Em4wu4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    pub fn em4wu5(&self) -> Em4wu5R {
        Em4wu5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    pub fn em4wu6(&self) -> Em4wu6R {
        Em4wu6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    pub fn em4wu7(&self) -> Em4wu7R {
        Em4wu7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&self) -> Em4wu8R {
        Em4wu8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&self) -> Em4wu9R {
        Em4wu9R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&mut self) -> Em4wu0W<'_, ExtilevelSpec> {
        Em4wu0W::new(self, 16)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&mut self) -> Em4wu1W<'_, ExtilevelSpec> {
        Em4wu1W::new(self, 17)
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    pub fn em4wu2(&mut self) -> Em4wu2W<'_, ExtilevelSpec> {
        Em4wu2W::new(self, 18)
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    pub fn em4wu3(&mut self) -> Em4wu3W<'_, ExtilevelSpec> {
        Em4wu3W::new(self, 19)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&mut self) -> Em4wu4W<'_, ExtilevelSpec> {
        Em4wu4W::new(self, 20)
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    pub fn em4wu5(&mut self) -> Em4wu5W<'_, ExtilevelSpec> {
        Em4wu5W::new(self, 21)
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    pub fn em4wu6(&mut self) -> Em4wu6W<'_, ExtilevelSpec> {
        Em4wu6W::new(self, 22)
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    pub fn em4wu7(&mut self) -> Em4wu7W<'_, ExtilevelSpec> {
        Em4wu7W::new(self, 23)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&mut self) -> Em4wu8W<'_, ExtilevelSpec> {
        Em4wu8W::new(self, 24)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&mut self) -> Em4wu9W<'_, ExtilevelSpec> {
        Em4wu9W::new(self, 25)
    }
}
#[doc = "External Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extilevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extilevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtilevelSpec;
impl crate::RegisterSpec for ExtilevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extilevel::R`](R) reader structure"]
impl crate::Readable for ExtilevelSpec {}
#[doc = "`write(|w| ..)` method takes [`extilevel::W`](W) writer structure"]
impl crate::Writable for ExtilevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTILEVEL to value 0"]
impl crate::Resettable for ExtilevelSpec {}
