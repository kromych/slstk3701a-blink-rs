#[doc = "Register `RAM1CTRL` reader"]
pub type R = crate::R<Ram1ctrlSpec>;
#[doc = "Register `RAM1CTRL` writer"]
pub type W = crate::W<Ram1ctrlSpec>;
#[doc = "RAM1 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampowerdown {
    #[doc = "0: None of the RAM blocks powered down"]
    None = 0,
    #[doc = "128: Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    Blk7 = 128,
    #[doc = "192: Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    Blk6to7 = 192,
    #[doc = "224: Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    Blk5to7 = 224,
    #[doc = "240: Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    Blk4to7 = 240,
    #[doc = "248: Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    Blk3to7 = 248,
    #[doc = "252: Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    Blk2to7 = 252,
    #[doc = "254: Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    Blk1to7 = 254,
    #[doc = "255: Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    Blk0to7 = 255,
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
#[doc = "Field `RAMPOWERDOWN` reader - RAM1 Blockset Power-down"]
pub type RampowerdownR = crate::FieldReader<Rampowerdown>;
impl RampowerdownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampowerdown> {
        match self.bits {
            0 => Some(Rampowerdown::None),
            128 => Some(Rampowerdown::Blk7),
            192 => Some(Rampowerdown::Blk6to7),
            224 => Some(Rampowerdown::Blk5to7),
            240 => Some(Rampowerdown::Blk4to7),
            248 => Some(Rampowerdown::Blk3to7),
            252 => Some(Rampowerdown::Blk2to7),
            254 => Some(Rampowerdown::Blk1to7),
            255 => Some(Rampowerdown::Blk0to7),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rampowerdown::None
    }
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk7(&self) -> bool {
        *self == Rampowerdown::Blk7
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk6to7(&self) -> bool {
        *self == Rampowerdown::Blk6to7
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk5to7(&self) -> bool {
        *self == Rampowerdown::Blk5to7
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk4to7(&self) -> bool {
        *self == Rampowerdown::Blk4to7
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk3to7(&self) -> bool {
        *self == Rampowerdown::Blk3to7
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk2to7(&self) -> bool {
        *self == Rampowerdown::Blk2to7
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk1to7(&self) -> bool {
        *self == Rampowerdown::Blk1to7
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk0to7(&self) -> bool {
        *self == Rampowerdown::Blk0to7
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM1 Blockset Power-down"]
pub type RampowerdownW<'a, REG> = crate::FieldWriter<'a, REG, 8, Rampowerdown>;
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
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk7)
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk6to7)
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk5to7)
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk4to7)
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk3to7)
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk2to7)
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk1to7)
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk0to7)
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RampowerdownR {
        RampowerdownR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RampowerdownW<'_, Ram1ctrlSpec> {
        RampowerdownW::new(self, 0)
    }
}
#[doc = "Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram1ctrlSpec;
impl crate::RegisterSpec for Ram1ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1ctrl::R`](R) reader structure"]
impl crate::Readable for Ram1ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram1ctrl::W`](W) writer structure"]
impl crate::Writable for Ram1ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM1CTRL to value 0"]
impl crate::Resettable for Ram1ctrlSpec {}
