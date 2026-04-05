#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DaintmskSpec>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DaintmskSpec>;
#[doc = "Field `INEPMSK0` reader - IN Endpoint 0 Interrupt mask Bit"]
pub type Inepmsk0R = crate::BitReader;
#[doc = "Field `INEPMSK0` writer - IN Endpoint 0 Interrupt mask Bit"]
pub type Inepmsk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK1` reader - IN Endpoint 1 Interrupt mask Bit"]
pub type Inepmsk1R = crate::BitReader;
#[doc = "Field `INEPMSK1` writer - IN Endpoint 1 Interrupt mask Bit"]
pub type Inepmsk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK2` reader - IN Endpoint 2 Interrupt mask Bit"]
pub type Inepmsk2R = crate::BitReader;
#[doc = "Field `INEPMSK2` writer - IN Endpoint 2 Interrupt mask Bit"]
pub type Inepmsk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK3` reader - IN Endpoint 3 Interrupt mask Bit"]
pub type Inepmsk3R = crate::BitReader;
#[doc = "Field `INEPMSK3` writer - IN Endpoint 3 Interrupt mask Bit"]
pub type Inepmsk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK4` reader - IN Endpoint 4 Interrupt mask Bit"]
pub type Inepmsk4R = crate::BitReader;
#[doc = "Field `INEPMSK4` writer - IN Endpoint 4 Interrupt mask Bit"]
pub type Inepmsk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK5` reader - IN Endpoint 5 Interrupt mask Bit"]
pub type Inepmsk5R = crate::BitReader;
#[doc = "Field `INEPMSK5` writer - IN Endpoint 5 Interrupt mask Bit"]
pub type Inepmsk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPMSK6` reader - IN Endpoint 6 Interrupt mask Bit"]
pub type Inepmsk6R = crate::BitReader;
#[doc = "Field `INEPMSK6` writer - IN Endpoint 6 Interrupt mask Bit"]
pub type Inepmsk6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK0` reader - OUT Endpoint 0 Interrupt mask Bit"]
pub type Outepmsk0R = crate::BitReader;
#[doc = "Field `OUTEPMSK0` writer - OUT Endpoint 0 Interrupt mask Bit"]
pub type Outepmsk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK1` reader - OUT Endpoint 1 Interrupt mask Bit"]
pub type Outepmsk1R = crate::BitReader;
#[doc = "Field `OUTEPMSK1` writer - OUT Endpoint 1 Interrupt mask Bit"]
pub type Outepmsk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK2` reader - OUT Endpoint 2 Interrupt mask Bit"]
pub type Outepmsk2R = crate::BitReader;
#[doc = "Field `OUTEPMSK2` writer - OUT Endpoint 2 Interrupt mask Bit"]
pub type Outepmsk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK3` reader - OUT Endpoint 3 Interrupt mask Bit"]
pub type Outepmsk3R = crate::BitReader;
#[doc = "Field `OUTEPMSK3` writer - OUT Endpoint 3 Interrupt mask Bit"]
pub type Outepmsk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK4` reader - OUT Endpoint 4 Interrupt mask Bit"]
pub type Outepmsk4R = crate::BitReader;
#[doc = "Field `OUTEPMSK4` writer - OUT Endpoint 4 Interrupt mask Bit"]
pub type Outepmsk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK5` reader - OUT Endpoint 5 Interrupt mask Bit"]
pub type Outepmsk5R = crate::BitReader;
#[doc = "Field `OUTEPMSK5` writer - OUT Endpoint 5 Interrupt mask Bit"]
pub type Outepmsk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK6` reader - OUT Endpoint 6 Interrupt mask Bit"]
pub type Outepmsk6R = crate::BitReader;
#[doc = "Field `OUTEPMSK6` writer - OUT Endpoint 6 Interrupt mask Bit"]
pub type Outepmsk6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk0(&self) -> Inepmsk0R {
        Inepmsk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk1(&self) -> Inepmsk1R {
        Inepmsk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk2(&self) -> Inepmsk2R {
        Inepmsk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk3(&self) -> Inepmsk3R {
        Inepmsk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Endpoint 4 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk4(&self) -> Inepmsk4R {
        Inepmsk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Endpoint 5 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk5(&self) -> Inepmsk5R {
        Inepmsk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint 6 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk6(&self) -> Inepmsk6R {
        Inepmsk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk0(&self) -> Outepmsk0R {
        Outepmsk0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk1(&self) -> Outepmsk1R {
        Outepmsk1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk2(&self) -> Outepmsk2R {
        Outepmsk2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk3(&self) -> Outepmsk3R {
        Outepmsk3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OUT Endpoint 4 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk4(&self) -> Outepmsk4R {
        Outepmsk4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OUT Endpoint 5 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk5(&self) -> Outepmsk5R {
        Outepmsk5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OUT Endpoint 6 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk6(&self) -> Outepmsk6R {
        Outepmsk6R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk0(&mut self) -> Inepmsk0W<'_, DaintmskSpec> {
        Inepmsk0W::new(self, 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk1(&mut self) -> Inepmsk1W<'_, DaintmskSpec> {
        Inepmsk1W::new(self, 1)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk2(&mut self) -> Inepmsk2W<'_, DaintmskSpec> {
        Inepmsk2W::new(self, 2)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk3(&mut self) -> Inepmsk3W<'_, DaintmskSpec> {
        Inepmsk3W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Endpoint 4 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk4(&mut self) -> Inepmsk4W<'_, DaintmskSpec> {
        Inepmsk4W::new(self, 4)
    }
    #[doc = "Bit 5 - IN Endpoint 5 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk5(&mut self) -> Inepmsk5W<'_, DaintmskSpec> {
        Inepmsk5W::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint 6 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk6(&mut self) -> Inepmsk6W<'_, DaintmskSpec> {
        Inepmsk6W::new(self, 6)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk0(&mut self) -> Outepmsk0W<'_, DaintmskSpec> {
        Outepmsk0W::new(self, 16)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk1(&mut self) -> Outepmsk1W<'_, DaintmskSpec> {
        Outepmsk1W::new(self, 17)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk2(&mut self) -> Outepmsk2W<'_, DaintmskSpec> {
        Outepmsk2W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk3(&mut self) -> Outepmsk3W<'_, DaintmskSpec> {
        Outepmsk3W::new(self, 19)
    }
    #[doc = "Bit 20 - OUT Endpoint 4 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk4(&mut self) -> Outepmsk4W<'_, DaintmskSpec> {
        Outepmsk4W::new(self, 20)
    }
    #[doc = "Bit 21 - OUT Endpoint 5 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk5(&mut self) -> Outepmsk5W<'_, DaintmskSpec> {
        Outepmsk5W::new(self, 21)
    }
    #[doc = "Bit 22 - OUT Endpoint 6 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk6(&mut self) -> Outepmsk6W<'_, DaintmskSpec> {
        Outepmsk6W::new(self, 22)
    }
}
#[doc = "Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintmskSpec;
impl crate::RegisterSpec for DaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DaintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DaintmskSpec {}
