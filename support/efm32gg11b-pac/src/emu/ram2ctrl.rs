#[doc = "Register `RAM2CTRL` reader"]
pub type R = crate::R<Ram2ctrlSpec>;
#[doc = "Register `RAM2CTRL` writer"]
pub type W = crate::W<Ram2ctrlSpec>;
#[doc = "RAM2 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampowerdown {
    #[doc = "0: None of the RAM blocks powered down"]
    None = 0,
    #[doc = "8: Power down RAM block 3"]
    Blk3 = 8,
    #[doc = "12: Power down RAM blocks 2-3"]
    Blk2to3 = 12,
    #[doc = "14: Power down RAM blocks 1-3"]
    Blk1to3 = 14,
    #[doc = "15: Power down RAM blocks 0-3"]
    Blk0to3 = 15,
}
impl From<Rampowerdown> for u8 {
    #[inline(always)]
    fn from(variant: Rampowerdown) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampowerdown {
    type Ux = u8;
}
impl crate::IsEnum for Rampowerdown {}
#[doc = "Field `RAMPOWERDOWN` reader - RAM2 Blockset Power-down"]
pub type RampowerdownR = crate::FieldReader<Rampowerdown>;
impl RampowerdownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampowerdown> {
        match self.bits {
            0 => Some(Rampowerdown::None),
            8 => Some(Rampowerdown::Blk3),
            12 => Some(Rampowerdown::Blk2to3),
            14 => Some(Rampowerdown::Blk1to3),
            15 => Some(Rampowerdown::Blk0to3),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rampowerdown::None
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == Rampowerdown::Blk3
    }
    #[doc = "Power down RAM blocks 2-3"]
    #[inline(always)]
    pub fn is_blk2to3(&self) -> bool {
        *self == Rampowerdown::Blk2to3
    }
    #[doc = "Power down RAM blocks 1-3"]
    #[inline(always)]
    pub fn is_blk1to3(&self) -> bool {
        *self == Rampowerdown::Blk1to3
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn is_blk0to3(&self) -> bool {
        *self == Rampowerdown::Blk0to3
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM2 Blockset Power-down"]
pub type RampowerdownW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rampowerdown>;
impl<'a, REG> RampowerdownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::None)
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk3)
    }
    #[doc = "Power down RAM blocks 2-3"]
    #[inline(always)]
    pub fn blk2to3(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk2to3)
    }
    #[doc = "Power down RAM blocks 1-3"]
    #[inline(always)]
    pub fn blk1to3(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk1to3)
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn blk0to3(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk0to3)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RampowerdownR {
        RampowerdownR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RampowerdownW<'_, Ram2ctrlSpec> {
        RampowerdownW::new(self, 0)
    }
}
#[doc = "Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram2ctrlSpec;
impl crate::RegisterSpec for Ram2ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram2ctrl::R`](R) reader structure"]
impl crate::Readable for Ram2ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram2ctrl::W`](W) writer structure"]
impl crate::Writable for Ram2ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM2CTRL to value 0"]
impl crate::Resettable for Ram2ctrlSpec {}
